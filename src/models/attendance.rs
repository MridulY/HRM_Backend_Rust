use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::models::employee::*;

const DATE_FORMAT: &str = "%Y-%m-%d";

fn date_format_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let option: Option<String> = Option::deserialize(deserializer)?;
    match option {
        Some(date_str) => NaiveDate::parse_from_str(&date_str, DATE_FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

// Custom serialization function
fn date_format_ser_opt<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.format(DATE_FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attendance {
    pub employee_id: String,
    #[serde(deserialize_with = "date_format", serialize_with = "date_format_ser")]
    pub date: NaiveDate,  
    #[serde(deserialize_with = "date_format", serialize_with = "date_format_ser")]
    pub punch_in: NaiveDate,
    #[serde(deserialize_with = "date_format_opt", serialize_with = "date_format_ser_opt")]
    pub punch_out: Option<NaiveDate>, 
    pub total_hours: Option<f64>,  
}
