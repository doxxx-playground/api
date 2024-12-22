use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use api::metrics::PrometheusMetrics;
use api::{db, handlers};
use dotenvy::dotenv;
use prometheus::{Encoder, TextEncoder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let db_pool = db::establish_connection(&database_url)
        .await
        .expect("Failed to create DB pool");

    let prometheus_metrics = PrometheusMetrics::new();

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus_metrics.clone()) // 미들웨어로 등록
            .app_data(web::Data::new(db_pool.clone()))
            .route("/health", web::get().to(handlers::health_check))
            .route("/metrics", web::get().to(metrics_handler))
            .service(
                web::scope("/items")
                    .route("", web::get().to(handlers::get_items))
                    .route("", web::post().to(handlers::create_item))
                    .route("/{id}", web::get().to(handlers::get_item_by_id))
                    .route("/{id}", web::put().to(handlers::update_item))
                    .route("/{id}", web::delete().to(handlers::delete_item)),
            )
    })
    .bind((host, port))?
    .run()
    .await
}

async fn metrics_handler() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(String::from_utf8(buffer).unwrap())
}
