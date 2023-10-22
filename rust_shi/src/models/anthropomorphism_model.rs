use crate::schema;
use diesel::{pg::Pg, Queryable, Selectable, Insertable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone)]
#[ExistingTypePath = "schema::sql_types::HairColorEnum"]
#[serde(rename_all = "lowercase")]
pub enum HairColor {
    Turquoise,
    Pink,
    Green,
    Yellow,
    Blue,
}

#[derive(DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "schema::sql_types::GenderEnum"]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::anthropomorphism)]
#[diesel(check_for_backend(Pg))]
pub struct Anthropomorphism {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
    pub gender: Gender,
    pub hair_color: HairColor,
}
