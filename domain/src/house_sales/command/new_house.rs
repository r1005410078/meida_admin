use bigdecimal::BigDecimal;

use crate::{
    house_sales::{aggregates::sales::ScalesAggregate, events::house::NewHouseEvent},
    repositories::sales_repository::SalesRepository,
};

pub struct NewHouseCommand {
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

pub struct NewHouseCommandHandler<R: SalesRepository> {
    repository: R,
}

impl<R: SalesRepository> NewHouseCommandHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: &NewHouseCommand) -> Result<NewHouseEvent, String> {
        if self
            .repository
            .not_exist_residential(command.neighborhood_id)
            .await
        {
            return Err("小区不存在".to_string());
        }

        if self
            .repository
            .exist_house(command.neighborhood_id, &command.house_address)
            .await
        {
            return Err("房屋已经存在".to_string());
        }

        let scales =
            ScalesAggregate::new_house(command.neighborhood_id, command.house_address.clone());

        self.repository
            .save(&scales)
            .await
            .map_err(|e| e.to_string())?;

        Ok(NewHouseEvent {
            neighborhood_id: command.neighborhood_id,
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
        })
    }
}
