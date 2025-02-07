use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateItemRequestDto {
    name: String,
    quantity: i16,
    storage_area: String,
}

impl CreateItemRequestDto {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> i16 {
        self.quantity
    }

    pub fn storage_area(&self) -> &str {
        &self.storage_area
    }
}