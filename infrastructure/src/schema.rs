// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Year"))]
    pub struct ResidentialYearBuiltYear;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ResidentialYearBuiltYear;

    residential (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 20]
        postal_code -> Varchar,
        year_built -> ResidentialYearBuiltYear,
        #[max_length = 100]
        community_type -> Varchar,
        description -> Nullable<Text>,
    }
}
