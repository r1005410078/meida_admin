use crate::house::{
    entities::house::{House, NewHouse, UpdateHouse},
    repositories::house::HouseRepository,
};

pub struct HouseService<T: HouseRepository> {
    pub house_repository: T,
}

impl<T: HouseRepository> HouseService<T> {
    pub fn new(house_repository: T) -> Self {
        Self { house_repository }
    }

    pub async fn get_all(&self) -> Vec<House> {
        self.house_repository.get_all().await
    }

    pub async fn create(&self, house: &NewHouse) -> Result<(), String> {
        if self
            .house_repository
            .exist(house.house_address.clone(), house.neighborhood_name.clone())
            .await
        {
            Err("该房源已存在".to_string())
        } else {
            self.house_repository
                .create(house)
                .await
                .map_err(|err| err.to_string())?;

            Ok(())
        }
    }

    pub async fn update(&self, house: &UpdateHouse) -> Result<(), diesel::result::Error> {
        self.house_repository.update(house).await
    }
}
