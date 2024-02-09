use crate::models::time_record::TimeRecord;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user: String,
    pub time_record: Vec<TimeRecord>,
}
