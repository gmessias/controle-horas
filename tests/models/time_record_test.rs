use super::*;

#[test]
fn test_create_valid_time_record() {
    let record = TimeRecord {
        id: "123".to_string(),
        date: NaiveDate::from_ymd(2024, 2, 7),
        hours: 10,
        minutes: 30,
    };

    assert_eq!(record.id, "123");
    assert_eq!(record.date, NaiveDate::from_ymd(2024, 2, 7));
    assert_eq!(record.hours, 10);
    assert_eq!(record.minutes, 30);
}

#[test]
fn test_invalid_id() {
    let record = TimeRecord {
        id: 123,
        date: NaiveDate::from_ymd(2024, 2, 7),
        hours: 10,
        minutes: 30,
    };

    assert!(!record.id.is_empty());
}

#[test]
fn test_invalid_date() {
    let record = TimeRecord {
        id: "456".to_string(),
        date: NaiveDate::from_ymd(2024, 2, 30),
        hours: 10,
        minutes: 30,
    };

    assert!(!record.date.is_valid());
}

#[test]
fn test_valid_month() {
    let record = TimeRecord {
        id: "789".to_string(),
        date: NaiveDate::from_ymd(2024, 13, 15),
        hours: 10,
        minutes: 30,
    };

    assert!(record.date.month() >= 1 && record.date.month() <= 12);
}