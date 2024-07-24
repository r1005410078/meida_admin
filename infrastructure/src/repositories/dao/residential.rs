use crate::schema::residential::dsl::*;
use crate::{db::connection::DBPool, schema::residential};
use diesel::OptionalExtension;
use diesel::{
    prelude::{AsChangeset, Insertable},
    Queryable, RunQueryDsl,
};
use diesel::{ExpressionMethods, QueryDsl};
use domain::residential::entities::residential::{NewResidential, Residential, UpdateResidential};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = residential)]
pub struct NewResidentialDao {
    name: String,
    address: String,
    city: String,
    state: String,
    postal_code: String,
    year_built: i16,
    community_type: String,
    description: Option<String>,
}

impl From<&NewResidential> for NewResidentialDao {
    fn from(rl: &NewResidential) -> Self {
        NewResidentialDao {
            name: rl.name.clone(),
            address: rl.address.clone(),
            city: rl.city.clone(),
            state: rl.state.clone(),
            postal_code: rl.postal_code.clone(),
            year_built: rl.year_built.clone(),
            community_type: rl.community_type.clone(),
            description: rl.description.clone(),
        }
    }
}

impl NewResidentialDao {
    pub fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(residential::table)
            .values(self)
            .execute(&mut conn)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, AsChangeset)]
#[diesel(table_name = residential)]
pub struct UpdateResidentialDao {
    pub id: i32,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub description: Option<String>,
}

impl From<&UpdateResidential> for UpdateResidentialDao {
    fn from(rl: &UpdateResidential) -> Self {
        UpdateResidentialDao {
            id: rl.id,
            name: rl.name.clone(),
            address: rl.address.clone(),
            city: rl.city.clone(),
            state: rl.state.clone(),
            postal_code: rl.postal_code.clone(),
            year_built: rl.year_built,
            community_type: rl.community_type.clone(),
            description: rl.description.clone(),
        }
    }
}

impl UpdateResidentialDao {
    pub fn update(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::update(residential::table)
            .filter(id.eq(self.id))
            .set(self)
            .execute(&mut conn)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Queryable)]
pub struct ResidentialDao {
    id: i32,
    name: String,
    address: String,
    city: String,
    state: String,
    postal_code: String,
    year_built: i16,
    community_type: String,
    description: Option<String>,
}

impl From<ResidentialDao> for Residential {
    fn from(from_residential: ResidentialDao) -> Self {
        Self {
            id: from_residential.id,
            name: from_residential.name,
            address: from_residential.address,
            city: from_residential.city,
            state: from_residential.state,
            postal_code: from_residential.postal_code,
            year_built: from_residential.year_built,
            community_type: from_residential.community_type,
            description: from_residential.description,
        }
    }
}

impl ResidentialDao {
    pub fn find_by_name(pool: DBPool, input_name: &str) -> Option<Residential> {
        let mut conn = pool.get().unwrap();

        Some(
            residential
                .filter(name.eq(input_name))
                .first::<ResidentialDao>(&mut conn)
                .optional()
                .expect("Error loading user")?
                .into(),
        )
    }

    pub fn find_all_residential(pool: DBPool) -> Vec<Residential> {
        let mut conn = pool.get().unwrap();
        let dto: Vec<ResidentialDao> = residential.load(&mut conn).expect("Error loading user");
        dto.into_iter().map(|x| x.into()).collect()
    }
}
