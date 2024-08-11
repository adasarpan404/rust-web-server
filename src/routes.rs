use crate::handlers::{
    analytics_handlers::aggregate_orders_with_items,
    item_handlers::{create_item, delete_item, get_all_items, get_item, update_item},
    order_handlers::{create_order, delete_order, get_order, update_order},
};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("/", web::post().to(create_item))
            .route("/", web::get().to(get_all_items))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::put().to(update_item))
            .route("/{id}", web::delete().to(delete_item)),
    )
    .service(
        web::scope("/orders")
            .route("/", web::post().to(create_order))
            .route("/{id}", web::get().to(get_order))
            .route("/{id}", web::put().to(update_order))
            .route("/{id}", web::delete().to(delete_order)),
    )
    .service(web::scope("/analytics").route("/", web::get().to(aggregate_orders_with_items)));
}
