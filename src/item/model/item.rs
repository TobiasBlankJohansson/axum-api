use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize,Deserialize,FromRow)]
pub struct Item{
    pub id: Uuid,
    pub name: String,
    pub quantity: i16,
    pub storage_area: String
}