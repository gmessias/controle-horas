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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::time_record::TimeRecord;
    use std::collections::HashMap;
    use chrono::NaiveDate;

    #[test]
    fn test_calculate_total_hours_per_date() {
        let mut records = HashMap::new();
        let date = NaiveDate::from_ymd_opt(2022, 7, 8);
        match date {
            Some(d) => {
                let record = TimeRecord {
                    id: "123".to_string(),
                    date: d,
                    hours: 8,
                    minutes: 70,
                };

                calculate_total_hours_per_date("2022-07-08".to_string(), &mut records, &record);

                assert_eq!(records.get("2022-07-08"), Some(&(9, 10)));
            },
            None => {
                panic!("Invalid date");
            }
        }
    }

    #[test]
    fn test_calculate_total_hours_per_id() {
        let mut records = HashMap::new();
        let date = NaiveDate::from_ymd_opt(2022, 7, 8);
        match date {
            Some(d) => {
                let record = TimeRecord {
                    id: "123".to_string(),
                    date: d,
                    hours: 8,
                    minutes: 70,
                };

                calculate_total_hours_per_id(&mut records, &record);

                assert_eq!(records.get("123"), Some(&(9, 10)));
            },
            None => {
                panic!("Invalid date");
            }
        }
    }

    #[test]
    fn test_convert_minutes_to_hours() {
        let mut hours = 1;
        let mut minutes = 70;

        convert_minutes_to_hours(&mut hours, &mut minutes);

        assert_eq!(hours, 2);
        assert_eq!(minutes, 10);
    }
}
