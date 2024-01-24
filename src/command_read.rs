use crate::opts::Read;
use crate::user::User;
use chrono::Datelike;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::{config_file_exists, read_config_file};

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

                        for record in &user.time_record {
                            if &record.id == id {
                                let date = record.date.format("%d/%m/%Y").to_string();
                                let total_hours = records.entry(date).or_insert(0);

                                *total_hours += record.hours;
                                total_hours_in_id += record.hours;
                            }
                        }

                        let mut dates: Vec<_> = records.keys().collect();
                        dates.sort();

                        println!("ID: {}", id);

                        for date in dates {
                            println!("\nData: {}", date);
                            println!("Total de horas: {}", records[date]);
                        }

                        println!("\nTotal de horas gastas neste ID: {}", total_hours_in_id);
                    } else if let Some(day) = &cr.day {
                        let day = NaiveDate::parse_from_str(day, "%d/%m/%Y").unwrap();
                        let mut records = HashMap::new();
                        let mut total_hours_in_day = 0;

                        for record in &user.time_record {
                            if record.date == day {
                                let id = record.id.clone();
                                let total_hours = records.entry(id).or_insert(0);

                                *total_hours += record.hours;
                                total_hours_in_day += record.hours;
                            }
                        }

                        println!("Data: {}", day.format("%d/%m/%Y"));

                        for (id, total_hours) in &records {
                            println!("\nID: {}", id);
                            println!("Total de horas: {}", total_hours);
                        }

                        println!("\nTotal de horas no dia: {}", total_hours_in_day);
                    } else if let Some(month) = &cr.month {
                        let month_year = NaiveDate::parse_from_str(&format!("01/{}", month), "%d/%m/%Y").unwrap();
                        let mut records = HashMap::new();

                        for record in &user.time_record {
                            if record.date.month() == month_year.month() && record.date.year() == month_year.year() {
                                let id = record.id.clone();
                                let total_hours = records.entry(id).or_insert(0);

                                *total_hours += record.hours;
                            }
                        }

                        println!("Mês: {}", month);

                        for (id, total_hours) in &records {
                            println!("\nID: {}", id);
                            println!("Total de horas: {}", total_hours);
                        }
                    }
                }
            },
            Err(e) => println!("Erro ao ler o arquivo de configuração: {}", e),
        }
    } else {
        println!("O arquivo de configuração não existe. Por favor, crie um usando o comando Create.");
    }
}