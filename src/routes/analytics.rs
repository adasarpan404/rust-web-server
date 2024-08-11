use crate::handlers::analytics_handlers::aggregate_orders_with_items;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/analytics").route("/", web::get().to(aggregate_orders_with_items)));
}
