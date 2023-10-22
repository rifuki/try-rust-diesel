use crate::schema;
use diesel::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "schema::sql_types::GenderEnum"]
pub enum Gender {
    Male,
    Female,
}

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "schema::sql_types::HairColorEnum"]
pub enum HairColor {
    Turquoise,
    Yellow,
    Pink,
    Blue,
    Green,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::characters)]
#[diesel(check_for_backend(Pg))]
pub struct Character {
    pub id: Uuid,
    pub code_name: String,
    pub name: String,
    pub age: i32,
}
