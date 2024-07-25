use domain::house::{
    entities::house::{NewHouse, UpdateHouse},
    repositories::house::HouseRepository,
    service::house::HouseService,
};

pub struct SaveHouseUseCase<T: HouseRepository> {
    house_service: HouseService<T>,
}

impl<T: HouseRepository> SaveHouseUseCase<T> {
    pub fn new(house_repo: T) -> Self {
        let house_service = HouseService::new(house_repo);
        Self { house_service }
    }
}

impl<T: HouseRepository> SaveHouseUseCase<T> {
    pub async fn execute_update(
        &self,
        input_house: &UpdateHouse,
    ) -> Result<(), diesel::result::Error> {
        self.house_service.update(input_house).await
    }

    pub async fn execute_create(&self, input_house: &NewHouse) -> Result<(), String> {
        self.house_service.create(input_house).await
    }
}
