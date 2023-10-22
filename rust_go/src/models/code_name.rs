use crate::schema;
use diesel::{Insertable, Queryable, Selectable, QueryableByName};

#[derive(Queryable, QueryableByName, Selectable, Insertable)]
#[diesel(table_name = schema::code_names)]
pub struct CodeName {
    pub id: String,
}

// impl Default for CodeName {
//     fn default() -> Self {
//         Self {
//             id: Default::default()
//         }
//     }
// }