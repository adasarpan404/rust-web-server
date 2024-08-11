use actix_web::{web, HttpResponse, Responder};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;

use crate::constants::ORDERS;
use crate::models::Order;

pub async fn aggregate_orders_with_items(db: web::Data<Database>) -> impl Responder {
    let orders_collection = db.collection::<Order>(ORDERS);

    let pipeline = vec![
        doc! {
            "$lookup": {
                "from": "items",
                "localField": "item_id",
                "foreignField": "_id",
                "as": "item_info",
            }
        },
        doc! {
            "$unwind": "$item_info"
        },
    ];

    let mut cursor = match orders_collection.aggregate(pipeline, None).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut results = Vec::new();

    while let Ok(Some(document)) = cursor.try_next().await {
        results.push(document);
    }

    if let Err(err) = cursor.try_next().await {
        eprintln!("Error fetching document: {}", err);
        return HttpResponse::InternalServerError().body("Error fetching document");
    }

    HttpResponse::Ok().json(results)
}
