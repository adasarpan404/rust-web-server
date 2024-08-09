use crate::models::Item;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn create_item(item: web::Json<Item>) -> impl Responder {
    let mut new_item = item.into_inner();
    new_item.id = Uuid::new_v4();
    HttpResponse::Ok().json(new_item)
}

pub async fn get_item(id: web::Path<Uuid>) -> impl Responder {
    let item = Item {
        id: *id,
        name: "Sample Item".to_string(),
        description: "This is a sample item".to_string(),
    };
    HttpResponse::Ok().json(item)
}

pub async fn update_item(id: web::Path<Uuid>, item: web::Json<Item>) -> impl Responder {
    let mut updated_item = item.into_inner();
    updated_item.id = *id;

    HttpResponse::Ok().json(updated_item)
}

pub async fn delete_item(id: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body(format!("Item {} deleted", id))
}
