use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub house_id: i32,
    pub neighborhood_id: i32,
    pub house_address: String,
    pub house_type: String,
    pub area: f64,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_by: i32,
    pub updated_by: i32,
    pub deleted_by: Option<i32>,
}

pub struct NewHouse {
    pub neighborhood_id: i32,
    pub house_address: String,
    pub house_type: String,
    pub area: f64,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
}
