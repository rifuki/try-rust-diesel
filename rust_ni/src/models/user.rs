use crate::schema;
use uuid::Uuid;
use diesel::{Queryable, Selectable, Insertable};
use diesel::pg::Pg;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone)]
#[ExistingTypePath = "schema::sql_types::UsersRoleEnum"]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    User
}


#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: UserRole
}

#[derive(Deserialize)]
pub struct CustomUserPayload {
    pub username: String,
    pub password: String,
    pub role: UserRole
}