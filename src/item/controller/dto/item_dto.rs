use serde::Serialize;
use uuid::Uuid;
use crate::item::model::item::Item;

#[derive(Serialize)]
pub struct ItemDto {
    id: Uuid,
    name: String,
    quantity: i16,
    storageArea: String
}
impl ItemDto{
    pub fn to_model(item: Item) -> ItemDto{
        ItemDto{
            id: item.id,
            name: item.name,
            quantity: item.quantity,
            storageArea: item.storage_area
        }
    }

    pub fn to_model_list(items: Vec<Item>) -> Vec<ItemDto>{
        let mut item_dto:Vec<ItemDto> = Vec::new();
        for item in items {
            item_dto.push(ItemDto::to_model(item))
        }
        item_dto
    }
}