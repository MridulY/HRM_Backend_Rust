use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::NaiveDate;

pub fn date_format<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(serde::de::Error::custom)
}

pub fn date_format_ser<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    date.format("%Y-%m-%d").to_string().serialize(serializer)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub name: String,
    pub email: String,
    pub position: String,
    pub department: String,
    pub contact: String,
    pub address: String,
    #[serde(deserialize_with = "date_format", serialize_with = "date_format_ser")]
    pub date_of_birth: NaiveDate,
}
