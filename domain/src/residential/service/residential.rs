use crate::residential::{
    entities::residential::{NewResidential, Residential, UpdateResidential},
    repositories::residential::ResidentialRepository,
};

#[derive(Clone)]
pub struct ResidentialService<T: ResidentialRepository> {
    residential_repo: T,
}

impl<T: ResidentialRepository> ResidentialService<T> {
    pub fn new(residential_repo: T) -> Self {
        Self { residential_repo }
    }

    pub async fn create_residential(
        &self,
        residential: &NewResidential,
    ) -> Result<(), diesel::result::Error> {
        self.residential_repo.create(residential).await?;

        Ok(())
    }

    pub async fn update_residential(
        &self,
        residential: &UpdateResidential,
    ) -> Result<(), diesel::result::Error> {
        self.residential_repo.update(residential).await?;

        Ok(())
    }

    pub async fn get_residential_by_name(&self, name: &str) -> Option<Residential> {
        self.residential_repo.get_residential_by_name(name).await
    }

    pub async fn get_all_residential(&self) -> Vec<Residential> {
        self.residential_repo.get_all_residential().await
    }
}
