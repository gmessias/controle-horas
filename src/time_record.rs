use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRecord {
    pub id: String,
    pub date: NaiveDate,
    pub hours: u8,
    pub minutes: u8,
}