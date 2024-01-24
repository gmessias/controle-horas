use crate::opts::Insert;
use crate::time_record::TimeRecord;
use crate::user::User;
use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;
use crate::config::{config_file_exists, read_config_file};

pub fn command_insert_time_recording(exe_path: &PathBuf, ci: &Insert) {
    if config_file_exists(&exe_path) {
        match read_config_file(&exe_path) {
            Ok(username) => {
                let mut path = exe_path.clone();
                path.push(format!("{}.json", username));

                if !path.exists() {
                    println!("Usuário não existe.");
                } else {
                    let data = fs::read_to_string(&path).unwrap();
                    let mut user: User = serde_json::from_str(&data).unwrap();

                    user.time_record.push(TimeRecord {
                        id: ci.id.clone(),
                        date: NaiveDate::parse_from_str(&ci.date, "%d/%m/%Y").unwrap(),
                        hours: ci.hours,
                    });

                    let json = serde_json::to_string_pretty(&user).unwrap();
                    fs::write(path, json).unwrap();

                    println!("Registro de horas inserido com sucesso.");
                }
            },
            Err(e) => println!("Erro ao ler o arquivo de configuração: {}", e),
        }
    } else {
        println!("O arquivo de configuração não existe. Por favor, crie um usando o comando Create.");
    }
}
