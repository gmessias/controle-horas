use chrono::NaiveDate;
use std::collections::HashMap;

pub fn print_id_result(id: &str, records: &HashMap<String, (u8, u8)>, hours: u8, minutes: u8) {
    let mut dates: Vec<_> = records.keys().collect();
    dates.sort();

    println!("ID: {}", id);

    for date in dates {
        print_total_date(date, records[date].0, records[date].1);
    }

    print_result("ID", hours, minutes);
}

pub fn print_day_result(day: &NaiveDate, records: &HashMap<String, (u8, u8)>, hours: u8, minutes: u8) {
    println!("Data: {}", day.format("%d/%m/%Y"));

    for (id, total_hours) in records {
        print_total_id(id, total_hours.0, total_hours.1);
    }

    print_result("DIA", hours, minutes);
}

pub fn print_month_result(month: &str, records: &HashMap<String, (u8, u8)>) {
    println!("Mês: {}", month);

    for (id, total_hours) in records {
        print_total_id(id, total_hours.0, total_hours.1);
    }
}

fn print_total_id(id: &str, hours: u8, minutes: u8) {
    println!();
    println!("ID: {}", id);
    println!("Total de HORAS: {}", hours);
    println!("Total de MINUTOS: {}", minutes);
}

fn print_total_date(date: &str, hours: u8, minutes: u8) {
    println!();
    println!("Data: {}", date);
    println!("Total de HORAS: {}", hours);
    println!("Total de MINUTOS: {}", minutes);
}

fn print_result(result: &str, hours: u8, minutes: u8) {
    println!();
    println!("Total de HORAS gastas {}: {}", result, hours);
    println!("Total de MINUTOS gastos {}: {}", result, minutes);
}