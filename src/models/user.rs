use serde::{Deserialize, Serialize};

use crate::models::time_record::TimeRecord;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user: String,
    pub time_record: Vec<TimeRecord>,
}
