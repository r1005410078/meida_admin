use super::dao::residential::{NewResidentialDao, ResidentialDao, UpdateResidentialDao};
use crate::db::connection::{establish_connection, DBPool};
use async_trait::async_trait;
use domain::residential::{
    entities::residential::{NewResidential, Residential, UpdateResidential},
    repositories::residential::ResidentialRepository,
};

pub struct MysqlResidentialRepository {
    pool: DBPool,
}

impl MysqlResidentialRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlResidentialRepository {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl ResidentialRepository for &MysqlResidentialRepository {
    async fn create(&self, residential: &NewResidential) -> Result<(), diesel::result::Error> {
        let dao: NewResidentialDao = residential.into();
        dao.create(self.pool.clone()).unwrap();
        Ok(())
    }

    async fn update(&self, residential: &UpdateResidential) -> Result<(), diesel::result::Error> {
        let dao: UpdateResidentialDao = residential.into();
        dao.update(self.pool.clone()).unwrap();
        Ok(())
    }

    async fn get_residential_by_name(&self, input_name: &str) -> Option<Residential> {
        ResidentialDao::find_by_name(self.pool.clone(), input_name)
    }
}
