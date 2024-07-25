use crate::schema::houses::dsl::*;
use crate::{db::connection::DBPool, schema::houses};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable},
    Queryable,
};
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl};

use diesel::QueryDsl;
use domain::house::entities::house::{House, NewHouse, UpdateHouse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct HouseDao {
    pub house_id: i32,
    pub neighborhood_name: String,
    pub house_address: String,
    pub house_type: String,
    pub area: BigDecimal,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_by: String,
    pub updated_by: String,
    pub deleted_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<HouseDao> for House {
    fn from(house: HouseDao) -> Self {
        Self {
            house_id: house.house_id,
            neighborhood_name: house.neighborhood_name,
            house_address: house.house_address,
            house_type: house.house_type,
            area: house.area.clone(),
            bedrooms: house.bedrooms,
            living_rooms: house.living_rooms,
            bathrooms: house.bathrooms,
            orientation: house.orientation,
            decoration_status: house.decoration_status,
            status: house.status,
            house_description: house.house_description,
            house_image: house.house_image,
            owner_name: house.owner_name,
            owner_phone: house.owner_phone,
            created_at: house.created_at,
            updated_at: house.updated_at,
            deleted_at: house.deleted_at,
            created_by: house.created_by,
            deleted_by: house.deleted_by,
        }
    }
}

impl HouseDao {
    pub fn list(pool: DBPool) -> Vec<House> {
        let mut conn = pool.get().unwrap();
        let dto: Vec<HouseDao> = houses
            .load::<HouseDao>(&mut conn)
            .expect("Error loading houses");

        dto.into_iter().map(|x| x.into()).collect()
    }

    pub fn exist(
        pool: DBPool,
        input_house_address: String,
        input_neighborhood_name: String,
    ) -> bool {
        let mut conn = pool.get().unwrap();

        houses
            .filter(
                house_address
                    .eq(input_house_address)
                    .and(neighborhood_name.eq(input_neighborhood_name)),
            )
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Error loading houses")
            > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = houses)]
pub struct NewHouseDao {
    pub neighborhood_name: String,
    pub house_address: String,
    pub house_type: String,
    pub area: BigDecimal,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
    pub created_by: String,
    pub updated_by: String,
}

impl From<&NewHouse> for NewHouseDao {
    fn from(house: &NewHouse) -> Self {
        Self {
            neighborhood_name: house.neighborhood_name.clone(),
            house_address: house.house_address.clone(),
            house_type: house.house_type.clone(),
            area: house.area.clone(),
            bedrooms: house.bedrooms,
            living_rooms: house.living_rooms,
            bathrooms: house.bathrooms,
            orientation: house.orientation.clone(),
            decoration_status: house.decoration_status.clone(),
            status: house.status.clone(),
            house_description: house.house_description.clone(),
            house_image: house.house_image.clone(),
            owner_name: house.owner_name.clone(),
            owner_phone: house.owner_phone.clone(),
            created_by: house.created_by.clone(),
            updated_by: house.updated_by.clone(),
        }
    }
}

impl NewHouseDao {
    pub fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(houses::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = houses)]
pub struct UpdateHouseDao {
    pub house_id: i32,
    pub neighborhood_name: Option<String>,
    pub house_address: Option<String>,
    pub house_type: Option<String>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,
}

impl From<&UpdateHouse> for UpdateHouseDao {
    fn from(house: &UpdateHouse) -> Self {
        Self {
            house_id: house.house_id,
            neighborhood_name: house.neighborhood_name.clone(),
            house_address: house.house_address.clone(),
            house_type: house.house_type.clone(),
            area: house.area.clone(),
            bedrooms: house.bedrooms,
            living_rooms: house.living_rooms,
            bathrooms: house.bathrooms,
            orientation: house.orientation.clone(),
            decoration_status: house.decoration_status.clone(),
            status: house.status.clone(),
            house_description: house.house_description.clone(),
            house_image: house.house_image.clone(),
            owner_name: house.owner_name.clone(),
            owner_phone: house.owner_phone.clone(),
        }
    }
}

impl UpdateHouseDao {
    pub fn update(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::update(houses.find(self.house_id))
            .set(self)
            .execute(&mut conn)
            .expect("Error updating house");

        Ok(())
    }
}
