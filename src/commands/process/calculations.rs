use crate::models::time_record::TimeRecord;
use std::collections::HashMap;

pub fn calculate_total_hours_per_date(_date: String, records: &mut HashMap<String, (u8, u8)>, record: &TimeRecord) {
    let total_hours = records.entry(_date).or_insert((0, 0));

    total_hours.0 += record.hours;
    total_hours.1 += record.minutes;

    if total_hours.1 >= 60 {
        total_hours.0 += total_hours.1 / 60;
        total_hours.1 %= 60;
    }
}

pub fn calculate_total_hours_per_id(records: &mut HashMap<String, (u8, u8)>, record: &TimeRecord) {
    let id = record.id.clone();
    let total_hours = records.entry(id).or_insert((0, 0));

    total_hours.0 += record.hours;
    total_hours.1 += record.minutes;

    if total_hours.1 >= 60 {
        total_hours.0 += total_hours.1 / 60;
        total_hours.1 %= 60;
    }
}
