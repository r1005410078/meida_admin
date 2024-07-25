// @generated automatically by Diesel CLI.

diesel::table! {
    houses (house_id) {
        house_id -> Integer,
        #[max_length = 255]
        neighborhood_name -> Varchar,
        #[max_length = 255]
        house_address -> Varchar,
        #[max_length = 50]
        house_type -> Varchar,
        area -> Decimal,
        bedrooms -> Integer,
        living_rooms -> Integer,
        bathrooms -> Integer,
        #[max_length = 20]
        orientation -> Nullable<Varchar>,
        #[max_length = 50]
        decoration_status -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        house_description -> Nullable<Text>,
        #[max_length = 255]
        house_image -> Nullable<Varchar>,
        #[max_length = 100]
        owner_name -> Varchar,
        #[max_length = 20]
        owner_phone -> Varchar,
        deleted_at -> Nullable<Datetime>,
        #[max_length = 255]
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
        #[max_length = 255]
        deleted_by -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
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
        year_built -> Smallint,
        #[max_length = 100]
        community_type -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(houses, residential,);
