use crate::{constants::ITEMS, models::Item};
use actix_web::{web, HttpResponse, Responder};
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

pub async fn create_item(db: web::Data<Database>, item: web::Json<Item>) -> impl Responder {
    let item_collection = db.collection::<Item>(ITEMS);
    let new_item = Item::new(item.name.clone(), item.description.clone());

    let insert_result = item_collection.insert_one(new_item.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(new_item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
pub async fn get_all_items(db: web::Data<Database>) -> impl Responder {
    let item_collection = db.collection::<Item>(ITEMS);
    let mut cursor = match item_collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut results = Vec::new();

    while let Ok(Some(document)) = cursor.try_next().await {
        results.push(document);
    }

    // Check for errors that may have occurred during the final `try_next` call
    if let Err(err) = cursor.try_next().await {
        eprintln!("Error fetching document: {}", err);
        return HttpResponse::InternalServerError().body("Error fetching document");
    }

    HttpResponse::Ok().json(results)
}

pub async fn get_item(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let item_collection = db.collection::<Item>(ITEMS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let item = item_collection.find_one(filter, None).await;

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
    let item_collection = db.collection::<Item>(ITEMS);

    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let filter = doc! { "_id": obj_id };
    let update = doc! { "$set": { "name": &item.name, "description": &item.description } };
    let update_result = item_collection.update_one(filter, update, None).await;
    match update_result {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().json(item.into_inner()),
        Ok(_) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_item(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let item_collection = db.collection::<Item>(ITEMS);
    let obj_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };
    let filter = doc! { "_id": obj_id };
    let delete_result = item_collection.delete_one(filter, None).await;
    match delete_result {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("Item deleted"),
        Ok(_) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
