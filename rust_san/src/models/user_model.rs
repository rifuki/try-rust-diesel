use crate::schema::{sql_types, users};
use diesel::{pg::Pg, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "sql_types::RoleEnum"]
#[serde(rename_all = "lowercase")]
pub enum RoleEnum {
    Admin,
    User,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: RoleEnum,
}

#[derive(Deserialize, Serialize)]
pub struct CustomUserPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<RoleEnum>,
}
