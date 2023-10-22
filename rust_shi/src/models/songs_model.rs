use crate::schema;
use uuid::Uuid;
use diesel::{Queryable, Selectable, pg::Pg};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::songs)]
#[diesel(check_for_backend(Pg))]
pub struct Songs {
    pub id: Uuid,
    pub anthropomorphic_id: Uuid,
    pub title: String,
    pub artist: String,
    pub release_year: i32
}