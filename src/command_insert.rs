use crate::opts::Insert;
use crate::registro_horas::RegistroHoras;
use crate::usuario::Usuario;
use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;

pub fn command_insert_time_recording(exe_path: &PathBuf, c: &Insert) {
    let mut path = exe_path.clone();
    path.push(format!("{}.json", c.usuario));

    if !path.exists() {
        println!("Usuário não existe.");
    } else {
        let data = fs::read_to_string(&path).unwrap();
        let mut usuario: Usuario = serde_json::from_str(&data).unwrap();
        usuario.registro_horas.push(RegistroHoras {
            id: c.id.clone(),
            data: NaiveDate::parse_from_str(&c.data, "%d/%m/%Y").unwrap(),
            horas: c.horas,
        });
        let json = serde_json::to_string_pretty(&usuario).unwrap();
        fs::write(path, json).unwrap();
        println!("Registro de horas inserido com sucesso.");
    }
}
