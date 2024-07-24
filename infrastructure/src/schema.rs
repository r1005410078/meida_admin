// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

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
        year_built -> Varchar,
        #[max_length = 100]
        community_type -> Varchar,
        description -> Nullable<Text>,
    }
}
