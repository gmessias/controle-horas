use crate::opts::Read;
use crate::usuario::Usuario;
use chrono::Datelike;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::{config_file_exists, read_config_file};

pub fn command_read_json(exe_path: &PathBuf, r: &Read) {
    if config_file_exists(&exe_path) {
        match read_config_file(&exe_path) {
            Ok(username) => {
                let mut path = exe_path.clone();
                path.push(format!("{}.json", username));

                if !path.exists() {
                    println!("Usuário não existe.");
                } else {
                    let data = fs::read_to_string(&path).unwrap();
                    let usuario: Usuario = serde_json::from_str(&data).unwrap();

                    if let Some(id) = &r.id {
                        let mut registros = HashMap::new();
                        let mut total_horas_no_id = 0;

                        for registro in &usuario.registro_horas {
                            if &registro.id == id {
                                let data = registro.data.format("%d/%m/%Y").to_string();
                                let total_horas = registros.entry(data).or_insert(0);
                                *total_horas += registro.horas;
                                total_horas_no_id += registro.horas;
                            }
                        }

                        let mut datas: Vec<_> = registros.keys().collect();
                        datas.sort();

                        println!("ID: {}", id);
                        for data in datas {
                            println!("\nData: {}", data);
                            println!("Total de horas: {}", registros[data]);
                        }
                        println!("\nTotal de horas gastas neste ID: {}", total_horas_no_id);
                    } else if let Some(dia) = &r.dia {
                        let dia = NaiveDate::parse_from_str(dia, "%d/%m/%Y").unwrap();
                        let mut registros = HashMap::new();
                        let mut total_horas_no_dia = 0;

                        for registro in &usuario.registro_horas {
                            if registro.data == dia {
                                let id = registro.id.clone();
                                let total_horas = registros.entry(id).or_insert(0);
                                *total_horas += registro.horas;
                                total_horas_no_dia += registro.horas;
                            }
                        }

                        println!("Data: {}", dia.format("%d/%m/%Y"));
                        for (id, total_horas) in &registros {
                            println!("\nID: {}", id);
                            println!("Total de horas: {}", total_horas);
                        }
                        println!("\nTotal de horas no dia: {}", total_horas_no_dia);
                    } else if let Some(mes) = &r.mes {
                        let mes_ano = NaiveDate::parse_from_str(&format!("01/{}", mes), "%d/%m/%Y").unwrap();
                        let mut registros = HashMap::new();

                        for registro in &usuario.registro_horas {
                            if registro.data.month() == mes_ano.month() && registro.data.year() == mes_ano.year() {
                                let id = registro.id.clone();
                                let total_horas = registros.entry(id).or_insert(0);
                                *total_horas += registro.horas;
                            }
                        }

                        println!("Mês: {}", mes);
                        for (id, total_horas) in &registros {
                            println!("\nID: {}", id);
                            println!("Total de horas: {}", total_horas);
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