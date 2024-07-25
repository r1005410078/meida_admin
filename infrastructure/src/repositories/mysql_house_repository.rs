use domain::house::{
    entities::house::{House, NewHouse, UpdateHouse},
    repositories::house::HouseRepository,
};

use crate::db::connection::{establish_connection, DBPool};

use super::dao::house::{HouseDao, NewHouseDao, UpdateHouseDao};

pub struct MySqlHouseRepository {
    pool: DBPool,
}

impl MySqlHouseRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MySqlHouseRepository {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait::async_trait]
impl HouseRepository for &MySqlHouseRepository {
    async fn exist(&self, address: String, neighborhood_name: String) -> bool {
        HouseDao::exist(self.pool.clone(), address, neighborhood_name)
    }

    async fn create(&self, input_house: &NewHouse) -> Result<(), diesel::result::Error> {
        let dao: NewHouseDao = input_house.into();
        dao.create(self.pool.clone())
    }

    async fn update(&self, input_house: &UpdateHouse) -> Result<(), diesel::result::Error> {
        let dao: UpdateHouseDao = input_house.into();
        dao.update(self.pool.clone())
    }

    async fn get_all(&self) -> Vec<House> {
        HouseDao::list(self.pool.clone())
    }
}
