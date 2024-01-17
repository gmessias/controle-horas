use crate::usuario::Usuario;
use std::fs;
use std::path::PathBuf;

pub fn command_create_user(exe_path: &PathBuf, usuario: &str) {
    let mut path = exe_path.clone();
    path.push(format!("{}.json", usuario));

    if path.exists() {
        println!("Já existe esse arquivo e usuário.");
    } else {
        let new_user = Usuario {
            usuario: usuario.to_string(),
            registro_horas: Vec::new(),
        };

        let json = serde_json::to_string_pretty(&new_user).unwrap();
        fs::write(path, json).unwrap();
        println!("Arquivo criado com sucesso.");
    }
}