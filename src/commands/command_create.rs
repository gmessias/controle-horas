use std::fs;
use std::path::PathBuf;

use crate::models::user::User;

pub fn command_create_user(exe_path: &PathBuf, user: &str) {
    let mut path = exe_path.clone();
    path.push(format!("{}.json", user));

    if path.exists() {
        println!("Já existe esse arquivo com este usuário.");
    } else {
        let new_user = User {
            user: user.to_string(),
            time_record: Vec::new(),
        };

        let json = serde_json::to_string_pretty(&new_user).unwrap();
        fs::write(path, json).unwrap();

        println!("Arquivo criado com sucesso.");
    }
}