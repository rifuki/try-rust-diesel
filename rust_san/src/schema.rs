// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_enum"))]
    pub struct RoleEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleEnum;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        role -> RoleEnum,
    }
}
