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
    use super::sql_types::GenderEnum;
    use super::sql_types::HairColorEnum;

    characters (id) {
        id -> Uuid,
        code_name -> Varchar,
        name -> Varchar,
        age -> Int4,
        gender -> GenderEnum,
        hair_color_enum -> HairColorEnum,
    }
}

diesel::table! {
    code_names (id) {
        id -> Varchar,
    }
}

diesel::joinable!(characters -> code_names (code_name));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    code_names,
);
