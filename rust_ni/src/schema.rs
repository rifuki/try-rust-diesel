// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "users_role_enum"))]
    pub struct UsersRoleEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersRoleEnum;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        role -> UsersRoleEnum,
    }
}
