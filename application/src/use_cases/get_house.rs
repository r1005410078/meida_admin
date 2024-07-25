use domain::house::{
    entities::house::House, repositories::house::HouseRepository, service::house::HouseService,
};

pub struct GetHouseUseCase<T: HouseRepository> {
    house_service: HouseService<T>,
}

impl<T: HouseRepository> GetHouseUseCase<T> {
    pub fn new(house_repo: T) -> Self {
        let house_service = HouseService::new(house_repo);
        Self { house_service }
    }

    pub async fn get(&self) -> Vec<House> {
        self.house_service.get_all().await
    }
}
