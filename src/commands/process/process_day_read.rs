use chrono::NaiveDate;
use crate::commands::process::calculations::calculate_total_hours_per_id;
use crate::commands::process::utils::print_day_result;
use crate::models::user::User;
use crate::opts::Read;
use std::collections::HashMap;

pub fn process_day(cr: &Read, user: &User) {
    if let Some(day) = &cr.day {
        let day = NaiveDate::parse_from_str(day, "%d/%m/%Y").unwrap();
        let mut records = HashMap::new();
        let mut total_hours_in_day = 0;
        let mut total_minutes_in_day = 0;

        for record in &user.time_record {
            if record.date == day {
                calculate_total_hours_per_id(&mut records, record);

                total_hours_in_day += record.hours;
                total_minutes_in_day += record.minutes;

                if total_minutes_in_day >= 60 {
                    total_hours_in_day += total_minutes_in_day / 60;
                    total_minutes_in_day %= 60;
                }
            }
        }

        print_day_result(&day, &records, total_hours_in_day, total_minutes_in_day);
    }
}