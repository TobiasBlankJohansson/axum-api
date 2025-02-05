use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Item{
    pub id: Uuid,
    pub name: String,
    pub quantity: u32,
    pub storage_area: String
}