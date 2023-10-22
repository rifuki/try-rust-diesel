use crate::{
    models::anthropomorphism_model::{Anthropomorphism, Gender, HairColor},
    schema::anthropomorphism::dsl::anthropomorphism as anthropomorphism_table,
};

use diesel::{pg::PgConnection, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

pub fn create_anthropomorphic(
    conn: &mut PgConnection,
    name: String,
    age: i32,
    gender: Gender,
    hair_color: HairColor,
) -> Result<Anthropomorphism, diesel::result::Error> {
    let new_anthropomorphic = Anthropomorphism {
        id: Uuid::new_v4(),
        name,
        age,
        gender,
        hair_color,
    };

    let result = diesel::insert_into(anthropomorphism_table)
        .values(new_anthropomorphic)
        .returning(Anthropomorphism::as_returning())
        .get_result(conn)?;

    Ok(result)
}
