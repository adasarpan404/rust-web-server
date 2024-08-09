use actix_web::{App, HttpServer};
mod db;
mod env;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::get_db().await;

    HttpServer::new(|| App::new().configure(routes::init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
