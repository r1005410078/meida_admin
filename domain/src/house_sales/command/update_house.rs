use bigdecimal::BigDecimal;

use crate::{
    house_sales::events::{event_queue::EventQueue, house::UpdateHouseEvent},
    repositories::sales_repository::SalesRepository,
};

pub struct UpdateHouseCommand {
    pub house_id: i32,
    pub neighborhood_id: i32,
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

pub struct UpdateHouseCommandHandler<R: SalesRepository, E: EventQueue> {
    repository: R,
    event_queue: E,
}

impl<R: SalesRepository, E: EventQueue> UpdateHouseCommandHandler<R, E> {
    pub fn new(repository: R, event_queue: E) -> Self {
        Self {
            repository,
            event_queue,
        }
    }

    pub async fn handle(&self, command: &UpdateHouseCommand) -> Result<(), String> {
        let mut sales = self
            .repository
            .find_by_id(command.house_id, command.neighborhood_id)
            .await;

        if let Some(sales) = &mut sales {
            sales.handle_update_house(&self.event_queue, command);
            Ok(())
        } else {
            Err("房源不存在".to_string())
        }
    }
}
