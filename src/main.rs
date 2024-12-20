use actix_web::{web, App, HttpServer};
use api::{db, handlers};
use dotenvy::dotenv;
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

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/health", web::get().to(handlers::health_check))
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
