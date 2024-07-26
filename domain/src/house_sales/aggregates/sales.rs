use serde::Serialize;

use crate::house_sales::{
    command::update_house::UpdateHouseCommand,
    events::{event_queue::EventQueue, house::UpdateHouseEvent},
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct ScalesAggregate {
    // 小区编号
    pub neighborhood_id: i32,
    // 房屋地址
    pub house_address: String,
    // 房源录入时间
    pub house_recorded_time: Option<i64>,
    // 二手房上架时间
    pub second_hand_house_time: Option<i64>,
    // 二手房卖出时间
    pub second_hand_house_sale_time: Option<i64>,
    // 二手房下架时间
    pub second_hand_house_down_time: Option<i64>,
}

impl ScalesAggregate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_house(neighborhood_id: i32, house_address: String) -> Self {
        let mut scale = Self::new();
        scale.house_recorded_time = Some(chrono::Utc::now().timestamp());
        scale.house_address = house_address;
        scale.neighborhood_id = neighborhood_id;
        scale
    }

    // 添加房源
    pub fn handle_update_house<T>(&mut self, event: &T, command: &UpdateHouseCommand)
    where
        T: EventQueue,
    {
        self.house_recorded_time = Some(chrono::Utc::now().timestamp());
        event.enqueue(UpdateHouseEvent {
            neighborhood_id: Some(command.neighborhood_id),
            house_address: command.house_address.clone(),
            house_type: command.house_type.clone(),
            area: command.area.clone(),
            bedrooms: command.bedrooms.clone(),
            living_rooms: command.living_rooms.clone(),
            bathrooms: command.bathrooms.clone(),
            orientation: command.orientation.clone(),
            decoration_status: command.decoration_status.clone(),
            status: command.status.clone(),
            house_description: command.house_description.clone(),
            house_image: command.house_image.clone(),
            owner_name: command.owner_name.clone(),
            owner_phone: command.owner_phone.clone(),
        });
    }

    pub fn exist_house(&self) -> bool {
        self.house_recorded_time.is_some()
    }
}
