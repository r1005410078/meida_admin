use bigdecimal::BigDecimal;

pub struct NewHouseEvent {
    pub neighborhood_id: i32,
    pub house_address: String,
    pub house_type: String,
    pub area: BigDecimal,
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

pub struct UpdateHouseEvent {
    pub neighborhood_id: Option<i32>,
    pub house_address: Option<String>,
    pub house_type: Option<String>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,
}
