use crate::models::time_record::TimeRecord;
use std::collections::HashMap;

pub fn calculate_total_hours_per_date(_date: String, records: &mut HashMap<String, (u8, u8)>, record: &TimeRecord) {
    let hours = records.entry(_date).or_insert((0, 0));

    hours.0 += record.hours;
    hours.1 += record.minutes;

    if hours.1 >= 60 {
        hours.0 += hours.1 / 60;
        hours.1 %= 60;
    }
}

pub fn calculate_total_hours_per_id(records: &mut HashMap<String, (u8, u8)>, record: &TimeRecord) {
    let id = record.id.clone();
    let hours = records.entry(id).or_insert((0, 0));

    hours.0 += record.hours;
    hours.1 += record.minutes;

    if hours.1 >= 60 {
        hours.0 += hours.1 / 60;
        hours.1 %= 60;
    }
}

pub fn convert_minutes_to_hours(hours: &mut u8, minutes: &mut u8) {
    if *minutes >= 60 {
        *hours += *minutes / 60;
        *minutes %= 60;
    }
}
