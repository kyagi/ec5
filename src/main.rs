use actix_web::{web, App, HttpServer};
use ec5::routes::{calculate, health_check};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/{instance_type}", web::get().to(calculate))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
