use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::employee::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Holiday {
    pub name: String,
    #[serde(deserialize_with = "date_format", serialize_with = "date_format_ser")]
    pub date: NaiveDate,  
    pub year: i32, 
}
