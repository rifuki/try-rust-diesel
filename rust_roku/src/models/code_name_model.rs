use crate::schema;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = schema::code_names)]
pub struct CodeName {
    #[serde(deserialize_with = "deserialize_lowercase")]
    pub id: String,
}

fn deserialize_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.to_lowercase())
}
