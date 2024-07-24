use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Residential {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewResidential {
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateResidential {
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
