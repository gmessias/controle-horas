use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistroHoras {
    pub id: String,
    pub data: NaiveDate,
    pub horas: u8,
}