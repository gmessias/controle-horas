use chrono::{Datelike, NaiveDate};
use crate::commands::process::calculations::calculate_total_hours_per_id;
use crate::commands::process::utils::print_month_result;
use crate::models::user::User;
use crate::opts::Read;
use std::collections::HashMap;

pub fn process_month(cr: &Read, user: &User) {
    if let Some(month) = &cr.month {
        let month_year = NaiveDate::parse_from_str(&format!("01/{}", month), "%d/%m/%Y").unwrap();
        let mut records = HashMap::new();

        for record in &user.time_record {
            if record.date.month() == month_year.month() && record.date.year() == month_year.year() {
                calculate_total_hours_per_id(&mut records, record);
            }
        }

        print_month_result(&month, &records);
    }
}