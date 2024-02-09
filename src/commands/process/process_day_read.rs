use chrono::NaiveDate;
use crate::commands::process::calculations::{calculate_total_hours_per_id, convert_minutes_to_hours};
use crate::commands::process::utils::print_day_result;
use crate::models::user::User;
use crate::opts::Read;
use std::collections::HashMap;

pub fn process_day(cr: &Read, user: &User) {
    if let Some(day) = &cr.day {
        let day = NaiveDate::parse_from_str(day, "%d/%m/%Y").unwrap();
        let mut records = HashMap::new();
        let mut hours = 0;
        let mut minutes = 0;

        for record in &user.time_record {
            if record.date == day {
                calculate_total_hours_per_id(&mut records, record);

                hours += record.hours;
                minutes += record.minutes;

                convert_minutes_to_hours(&mut hours, &mut minutes);
            }
        }

        print_day_result(&day, &records, hours, minutes);
    }
}