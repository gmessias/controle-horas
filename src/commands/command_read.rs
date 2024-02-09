use crate::commands::process::process_day_read::process_day;
use crate::commands::process::process_id_read::process_id;
use crate::commands::process::process_month_read::process_month;
use crate::config::{config_file_exists, read_config_file};
use crate::models::user::User;
use crate::opts::Read;
use std::fs;
use std::path::PathBuf;

pub fn command_read_json(exe_path: &PathBuf, cr: &Read) {
    if !config_file_exists(&exe_path) {
        println!("O arquivo de configuração não existe. Por favor, crie um usando o comando Create.");
        return;
    }

    match read_config_file(&exe_path) {
        Ok(username) => {
            let mut path = exe_path.clone();
            path.push(format!("{}.json", username));

            if !path.exists() {
                println!("Usuário não existe.");
                return;
            }

            let data = fs::read_to_string(&path).unwrap();
            let user: User = serde_json::from_str(&data).unwrap();

            process_id(cr, &user);
            process_day(cr, &user);
            process_month(cr, &user);
        }
        Err(e) => println!("Erro ao ler o arquivo de configuração: {}", e),
    }
}




