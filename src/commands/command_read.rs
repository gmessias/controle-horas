use chrono::Datelike;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::{config_file_exists, read_config_file};
use crate::opts::Read;
use crate::models::user::User;

pub fn command_read_json(exe_path: &PathBuf, cr: &Read) {
    if config_file_exists(&exe_path) {
        match read_config_file(&exe_path) {
            Ok(username) => {
                let mut path = exe_path.clone();
                path.push(format!("{}.json", username));

                if !path.exists() {
                    println!("Usuário não existe.");
                } else {
                    let data = fs::read_to_string(&path).unwrap();
                    let user: User = serde_json::from_str(&data).unwrap();

                    if let Some(id) = &cr.id {
                        let mut records = HashMap::new();
                        let mut total_hours_in_id = 0;
                        let mut total_minutes_in_id = 0;

                        for record in &user.time_record {
                            if &record.id == id {
                                let date = record.date.format("%d/%m/%Y").to_string();
                                let total_hours = records.entry(date).or_insert((0, 0));

                                total_hours.0 += record.hours;
                                total_hours.1 += record.minutes;

                                if total_hours.1 >= 60 {
                                    total_hours.0 += total_hours.1 / 60;
                                    total_hours.1 %= 60;
                                }

                                total_hours_in_id += record.hours;
                                total_minutes_in_id += record.minutes;

                                if total_minutes_in_id >= 60 {
                                    total_hours_in_id += total_minutes_in_id / 60;
                                    total_minutes_in_id %= 60;
                                }
                            }
                        }

                        print_id_result(&id, &records, total_hours_in_id, total_minutes_in_id);
                    } else if let Some(day) = &cr.day {
                        let day = NaiveDate::parse_from_str(day, "%d/%m/%Y").unwrap();
                        let mut records = HashMap::new();
                        let mut total_hours_in_day = 0;
                        let mut total_minutes_in_day = 0;

                        for record in &user.time_record {
                            if record.date == day {
                                let id = record.id.clone();
                                let total_hours = records.entry(id).or_insert((0, 0));

                                total_hours.0 += record.hours;
                                total_hours.1 += record.minutes;

                                if total_hours.1 >= 60 {
                                    total_hours.0 += total_hours.1 / 60;
                                    total_hours.1 %= 60;
                                }

                                total_hours_in_day += record.hours;
                                total_minutes_in_day += record.minutes;

                                if total_minutes_in_day >= 60 {
                                    total_hours_in_day += total_minutes_in_day / 60;
                                    total_minutes_in_day %= 60;
                                }
                            }
                        }

                        print_day_result(&day, &records, total_hours_in_day, total_minutes_in_day);
                    } else if let Some(month) = &cr.month {
                        let month_year = NaiveDate::parse_from_str(&format!("01/{}", month), "%d/%m/%Y").unwrap();
                        let mut records = HashMap::new();

                        for record in &user.time_record {
                            if record.date.month() == month_year.month() && record.date.year() == month_year.year() {
                                let id = record.id.clone();
                                let total_hours = records.entry(id).or_insert((0, 0));

                                total_hours.0 += record.hours;
                                total_hours.1 += record.minutes;

                                if total_hours.1 >= 60 {
                                    total_hours.0 += total_hours.1 / 60;
                                    total_hours.1 %= 60;
                                }
                            }
                        }

                        print_month_result(&month, &records);
                    }
                }
            }
            Err(e) => println!("Erro ao ler o arquivo de configuração: {}", e),
        }
    } else {
        println!("O arquivo de configuração não existe. Por favor, crie um usando o comando Create.");
    }
}

fn print_id_result(id: &str, records: &HashMap<String, (u8, u8)>, total_hours_in_id: u8, total_minutes_in_id: u8) {
    let mut dates: Vec<_> = records.keys().collect();
    dates.sort();

    println!("ID: {}", id);

    for date in dates {
        println!("\nData: {}", date);
        println!("Total de horas: {}", records[date].0);
        println!("Total de minutos: {}", records[date].1);
    }

    println!("\nTotal de horas gastas neste ID: {}", total_hours_in_id);
    println!("Total de minutos gastos neste ID: {}", total_minutes_in_id);
}

fn print_day_result(day: &NaiveDate, records: &HashMap<String, (u8, u8)>, total_hours_in_day: u8, total_minutes_in_day: u8) {
    println!("Data: {}", day.format("%d/%m/%Y"));

    for (id, total_hours) in records {
        println!("\nID: {}", id);
        println!("Total de horas: {}", total_hours.0);
        println!("Total de minutos: {}", total_hours.1);
    }

    println!("\nTotal de horas no dia: {}", total_hours_in_day);
    println!("Total de minutos no dia: {}", total_minutes_in_day);
}

fn print_month_result(month: &str, records: &HashMap<String, (u8, u8)>) {
    println!("Mês: {}", month);

    for (id, total_hours) in records {
        println!("\nID: {}", id);
        println!("Total de horas: {}", total_hours.0);
        println!("Total de minutos: {}", total_hours.1);
    }
}