use crate::commands::process::calculations::{calculate_total_hours_per_date, convert_minutes_to_hours};
use crate::commands::process::utils::print_id_result;
use crate::models::user::User;
use crate::opts::Read;
use std::collections::HashMap;

pub fn process_id(cr: &Read, user: &User) {
    if let Some(id) = &cr.id {
        let mut records = HashMap::new();
        let mut hours = 0;
        let mut minutes = 0;

        for record in &user.time_record {
            if &record.id == id {
                let _date = record.date.format("%d/%m/%Y").to_string();
                calculate_total_hours_per_date(_date, &mut records, record);

                hours += record.hours;
                minutes += record.minutes;

                convert_minutes_to_hours(&mut hours, &mut minutes);
            }
        }

        print_id_result(&id, &records, hours, minutes);
    }
}