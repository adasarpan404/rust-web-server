use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::{
    constants::{ITEMS, ORDERS},
    models::{Item, Order, OrderWithItem},
};

pub async fn create_order(db: web::Data<Database>, order: web::Json<Order>) -> impl Responder {
    let order_collection = db.collection::<Order>(ORDERS);

    let new_order = Order::new(order.item_id.clone(), order.quantity.clone());
    let insert_result = order_collection.insert_one(new_order.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(new_order),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_order(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let orders_collection = db.collection::<Order>(ORDERS);
    let items_collection = db.collection::<Item>(ITEMS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let order = match orders_collection.find_one(filter, None).await {
        Ok(Some(order)) => order,
        Ok(None) => return HttpResponse::NotFound().body("Order not found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let item_filter = doc! { "_id": order.item_id };
    let item = match items_collection.find_one(item_filter, None).await {
        Ok(Some(item)) => item,
        Ok(None) => return HttpResponse::NotFound().body("Item not found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let order_with_item = OrderWithItem {
        id: order.id,
        item,
        quantity: order.quantity,
    };

    HttpResponse::Ok().json(order_with_item)
}

pub async fn update_order(
    db: web::Data<Database>,
    id: web::Path<String>,
    order: web::Json<Order>,
) -> impl Responder {
    let order_collection = db.collection::<Order>(ORDERS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let update = doc! { "$set": { "item_id": &order.item_id, "quantity": &order.quantity } };

    let update_result = order_collection.update_one(filter, update, None).await;

    match update_result {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().json(order.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("Order not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_order(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let order_collection = db.collection::<Order>(ORDERS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let delete_result = order_collection.delete_one(filter, None).await;

    match delete_result {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("Order deleted"),
        Ok(_) => HttpResponse::NotFound().body("Order not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
