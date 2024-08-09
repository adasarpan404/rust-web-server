use crate::handlers::{create_item, delete_item, get_item, update_item};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("/", web::post().to(create_item))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::put().to(update_item))
            .route("/{id}", web::delete().to(delete_item)),
    );
}
