use crate::commands::process::calculations::calculate_total_hours_per_date;
use crate::commands::process::utils::print_id_result;
use crate::models::user::User;
use crate::opts::Read;
use std::collections::HashMap;

pub fn process_id(cr: &Read, user: &User) {
    if let Some(id) = &cr.id {
        let mut records = HashMap::new();
        let mut total_hours_in_id = 0;
        let mut total_minutes_in_id = 0;

        for record in &user.time_record {
            if &record.id == id {
                let _date = record.date.format("%d/%m/%Y").to_string();
                calculate_total_hours_per_date(_date, &mut records, record);

                total_hours_in_id += record.hours;
                total_minutes_in_id += record.minutes;

                if total_minutes_in_id >= 60 {
                    total_hours_in_id += total_minutes_in_id / 60;
                    total_minutes_in_id %= 60;
                }
            }
        }

        print_id_result(&id, &records, total_hours_in_id, total_minutes_in_id);
    }
}