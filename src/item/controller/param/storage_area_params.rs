use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, ToSchema, IntoParams)]
pub struct StorageAreaParams {
    storage_area: Option<String>,
}

impl StorageAreaParams {
    pub fn storage_area(&self) -> &Option<String> {
        &self.storage_area
    }
}