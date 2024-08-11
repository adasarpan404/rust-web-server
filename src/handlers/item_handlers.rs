use crate::{constants::ITEMS, models::Item};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

pub async fn create_item(db: web::Data<Database>, item: web::Json<Item>) -> impl Responder {
    let collection = db.collection::<Item>(ITEMS);
    let new_item = Item::new(item.name.clone(), item.description.clone());

    let insert_result = collection.insert_one(new_item.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(new_item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_item(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let collection = db.collection::<Item>(ITEMS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let item = collection.find_one(filter, None).await;

    match item {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_item(
    db: web::Data<Database>,
    id: web::Path<String>,
    item: web::Json<Item>,
) -> impl Responder {
    let collection = db.collection::<Item>(ITEMS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let update = doc! { "$set": { "name": &item.name, "description": &item.description } };
    let update_result = collection.update_one(filter, update, None).await;
    match update_result {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().json(item.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_item(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let collection = db.collection::<Item>(ITEMS);
    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };
    let filter = doc! { "_id": obj_id };
    let delete_result = collection.delete_one(filter, None).await;
    match delete_result {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("Item deleted"),
        Ok(_) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
