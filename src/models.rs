use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub item_id: ObjectId,
    pub quantity: i32,
}

impl Item {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
        }
    }
}

impl Order {
    pub fn new(item_id: ObjectId, quantity: i32) -> Self {
        Self {
            id: None,
            item_id,
            quantity,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderWithItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub item: Item,
    pub quantity: i32,
}
