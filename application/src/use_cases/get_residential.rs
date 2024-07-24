use domain::residential::{
    entities::residential::Residential, repositories::residential::ResidentialRepository,
    service::residential::ResidentialService,
};

pub struct GetResidentialUseCase<T: ResidentialRepository> {
    residential_service: ResidentialService<T>,
}

impl<T: ResidentialRepository> GetResidentialUseCase<T> {
    pub fn new(residential_repo: T) -> Self {
        let residential_service = ResidentialService::new(residential_repo);
        Self {
            residential_service,
        }
    }

    pub async fn get(&self, name: &str) -> Option<Residential> {
        self.residential_service.get_residential_by_name(name).await
    }

    pub async fn get_list(&self) -> Vec<Residential> {
        self.residential_service.get_all_residential().await
    }
}
