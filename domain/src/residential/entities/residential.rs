use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Residential {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: String,
    pub community_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewResidential {
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: String,
    pub community_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateResidential {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<String>,
    pub community_type: Option<String>,
    pub description: Option<String>,
}
