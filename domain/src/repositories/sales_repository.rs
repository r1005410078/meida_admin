use crate::house_sales::aggregates::sales::ScalesAggregate;
use async_trait::async_trait;

#[async_trait]
pub trait SalesRepository {
    async fn not_exist_residential(&self, neighborhood_id: i32) -> bool;
    async fn exist_house(&self, neighborhood_id: i32, house_address: &String) -> bool;
    async fn save(&self, scales: &ScalesAggregate) -> Result<(), diesel::result::Error>;
    async fn find_by_id(&self, house_id: i32, neighborhood_id: i32) -> Option<ScalesAggregate>;
}
