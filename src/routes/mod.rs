pub mod analytics;
pub mod items;
pub mod orders;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    items::init(cfg);
    orders::init(cfg);
    analytics::init(cfg);
}
