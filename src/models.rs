use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
