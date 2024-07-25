use crate::house::entities::house::{House, NewHouse, UpdateHouse};
use async_trait::async_trait;

#[async_trait]
pub trait HouseRepository {
    async fn create(&self, house: &NewHouse) -> Result<(), diesel::result::Error>;
    async fn update(&self, house: &UpdateHouse) -> Result<(), diesel::result::Error>;
    // 是否存在房源
    async fn exist(&self, address: String, neighborhood_name: String) -> bool;
    async fn get_all(&self) -> Vec<House>;
}
