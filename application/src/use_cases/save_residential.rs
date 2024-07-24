use domain::residential::{
    entities::residential::{NewResidential, UpdateResidential},
    repositories::residential::ResidentialRepository,
    service::residential::ResidentialService,
};

pub struct SaveResidentialUseCase<T: ResidentialRepository> {
    residential_service: ResidentialService<T>,
}

impl<T: ResidentialRepository> SaveResidentialUseCase<T> {
    pub fn new(residential_repo: T) -> Self {
        let residential_service = ResidentialService::new(residential_repo);
        Self {
            residential_service,
        }
    }

    pub async fn execute_create(
        &self,
        residential: &NewResidential,
    ) -> Result<(), diesel::result::Error> {
        self.residential_service
            .create_residential(residential)
            .await?;

        Ok(())
    }

    pub async fn execute_update(
        &self,
        residential: &UpdateResidential,
    ) -> Result<(), diesel::result::Error> {
        self.residential_service
            .update_residential(residential)
            .await?;

        Ok(())
    }
}
