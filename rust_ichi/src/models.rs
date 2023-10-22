use crate::schema;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(Pg))]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::posts)]
pub struct NewPost<'a> {
    pub id: Option<Uuid>,
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
