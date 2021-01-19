use crate::schema::apis;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ----------------------------------------------------------------------------------------------------
// USERS
// ----------------------------------------------------------------------------------------------------
#[table_name = "apis"]
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, QueryableByName, Insertable)]
pub struct Api {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub respository: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    #[serde(with = "date_format")]
    pub last_updated: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub created: DateTime<Utc>,
    pub creator_id: String,
}

#[table_name = "apis"]
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, QueryableByName, Insertable)]
pub struct NewApi {
    pub name: String,
    pub description: String,
    pub respository: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub creator_id: String,
}

mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
