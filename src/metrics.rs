use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use prometheus::{register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec};
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

#[derive(Clone)]
pub struct PrometheusMetrics {
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: HistogramVec,
}

impl PrometheusMetrics {
    pub fn new() -> Self {
        let http_requests_total = register_int_counter_vec!(
            "http_requests_total",
            "Total number of HTTP requests",
            &["method", "path", "status"]
        )
        .unwrap();

        let http_request_duration_seconds = register_histogram_vec!(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
            &["method", "path"],
            vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
        )
        .unwrap();

        PrometheusMetrics {
            http_requests_total,
            http_request_duration_seconds,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PrometheusMetrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PrometheusMetricsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PrometheusMetricsMiddleware {
            service,
            http_requests_total: self.http_requests_total.clone(),
            http_request_duration_seconds: self.http_request_duration_seconds.clone(),
        })
    }
}

pub struct PrometheusMetricsMiddleware<S> {
    service: S,
    http_requests_total: IntCounterVec,
    http_request_duration_seconds: HistogramVec,
}

impl<S, B> Service<ServiceRequest> for PrometheusMetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let requests_total = self.http_requests_total.clone();
        let request_duration = self.http_request_duration_seconds.clone();

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed().as_secs_f64();
            let status = res.status().as_u16().to_string();

            requests_total
                .with_label_values(&[&method, &path, &status])
                .inc();
            request_duration
                .with_label_values(&[&method, &path])
                .observe(duration);

            Ok(res)
        })
    }
}
