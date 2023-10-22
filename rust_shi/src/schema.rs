// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender_enum"))]
    pub struct GenderEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hair_color_enum"))]
    pub struct HairColorEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HairColorEnum;
    use super::sql_types::GenderEnum;

    anthropomorphism (id) {
        id -> Uuid,
        name -> Varchar,
        age -> Int4,
        hair_color -> HairColorEnum,
        gender -> GenderEnum,
    }
}

diesel::table! {
    songs (id) {
        id -> Uuid,
        anthropomorphic_id -> Uuid,
        title -> Varchar,
        artist -> Varchar,
        release_year -> Int4,
    }
}

diesel::joinable!(songs -> anthropomorphism (anthropomorphic_id));

diesel::allow_tables_to_appear_in_same_query!(
    anthropomorphism,
    songs,
);
