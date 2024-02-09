use chrono::NaiveDate;
use crate::config::{config_file_exists, read_config_file};
use crate::models::time_record::TimeRecord;
use crate::models::user::User;
use crate::opts::Insert;
use std::fs;
use std::path::PathBuf;

pub fn command_insert_time_recording(exe_path: &PathBuf, ci: &Insert) {
    let (hours, minutes) = parse_time(ci);

    if hours == 0 && minutes == 0 {
        println!("Erro: Você deve fornecer pelo menos horas ou minutos.");
        return;
    }

    if config_file_exists(&exe_path) {
        handle_config_file(&exe_path, &ci, hours, minutes);
    } else {
        println!("O arquivo de configuração não existe. Por favor, crie um usando o comando Create.");
    }
}

fn parse_time(ci: &Insert) -> (u8, u8) {
    let mut hours = ci.hours.unwrap_or(0);
    let mut minutes = ci.minutes.unwrap_or(0);

    if let Some(time) = &ci.time {
        if time.contains(':') {
            let parts: Vec<&str> = time.split(':').collect();
            hours += parts[0].parse::<u8>().unwrap_or(0);
            minutes += parts[1].parse::<u8>().unwrap_or(0);
        } else {
            hours += time.parse::<u8>().unwrap_or(0);
        }
    }

    (hours, minutes)
}

fn handle_config_file(exe_path: &PathBuf, ci: &Insert, hours: u8, minutes: u8) {
    match read_config_file(&exe_path) {
        Ok(username) => {
            let mut path = exe_path.clone();
            path.push(format!("{}.json", username));

            if !path.exists() {
                println!("Usuário não existe.");
            } else {
                insert_time_record(&path, &ci, hours, minutes);
            }
        }
        Err(e) => println!("Erro ao ler o arquivo de configuração: {}", e),
    }
}

fn insert_time_record(path: &PathBuf, ci: &Insert, mut hours: u8, mut minutes: u8) {
    let data = fs::read_to_string(&path).unwrap();
    let mut user: User = serde_json::from_str(&data).unwrap();

    if minutes >= 60 {
        hours += minutes / 60;
        minutes = minutes % 60;
    }

    user.time_record.push(TimeRecord {
        id: ci.id.clone(),
        date: NaiveDate::parse_from_str(&ci.date, "%d/%m/%Y").unwrap(),
        hours,
        minutes,
    });

    let json = serde_json::to_string_pretty(&user).unwrap();
    fs::write(path, json).unwrap();

    println!("Registro de horas inserido com sucesso.");
}
