use crate::residential::entities::residential::{NewResidential, Residential, UpdateResidential};
use async_trait::async_trait;

#[async_trait]
pub trait ResidentialRepository {
    async fn create(
        &self,
        create_residential: &NewResidential,
    ) -> Result<(), diesel::result::Error>;

    async fn update(
        &self,
        create_residential: &UpdateResidential,
    ) -> Result<(), diesel::result::Error>;

    async fn get_residential_by_name(&self, name: &str) -> Option<Residential>;

    async fn get_all_residential(&self) -> Vec<Residential>;
}
