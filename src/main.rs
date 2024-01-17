use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use chrono::NaiveDate;
use std::env;
use std::collections::HashMap;
use chrono::Datelike;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Create(Create),
    Insert(Insert),
    Read(Read),
}

#[derive(Parser, Debug)]
struct Create {
    #[clap(short, long)]
    usuario: String,
}

#[derive(Parser, Debug)]
struct Insert {
    #[clap(short, long)]
    usuario: String,
    #[clap(long)]
    id: String,
    #[clap(long)]
    data: String,
    #[clap(long)]
    horas: u8,
}

#[derive(Parser, Debug)]
struct Read {
    #[clap(short, long)]
    usuario: String,
    #[clap(long)]
    id: Option<String>,
    #[clap(long)]
    dia: Option<String>,
    #[clap(long)]
    mes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usuario {
    usuario: String,
    registro_horas: Vec<RegistroHoras>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegistroHoras {
    id: String,
    data: NaiveDate,
    horas: u8,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut exe_path = env::current_exe().unwrap();
    exe_path.pop();

    match opts.subcmd {
        Command::Create(c) => {
            let mut path = exe_path.clone();
            path.push(format!("{}.json", c.usuario));

            if Path::new(&path).exists() {
                println!("Já existe esse arquivo e usuário.");
            } else {
                let usuario = Usuario {
                    usuario: c.usuario,
                    registro_horas: Vec::new(),
                };
                let json = serde_json::to_string_pretty(&usuario).unwrap();
                fs::write(path, json).unwrap();
                println!("Arquivo criado com sucesso.");
            }
        },
        Command::Insert(c) => {
            let mut path = exe_path.clone();
            path.push(format!("{}.json", c.usuario));
            
            if !Path::new(&path).exists() {
                println!("Usuário não existe.");
            } else {
                let data = fs::read_to_string(&path).unwrap();
                let mut usuario: Usuario = serde_json::from_str(&data).unwrap();
                usuario.registro_horas.push(RegistroHoras {
                    id: c.id,
                    data: NaiveDate::parse_from_str(&c.data, "%d/%m/%Y").unwrap(),
                    horas: c.horas,
                });
                let json = serde_json::to_string_pretty(&usuario).unwrap();
                fs::write(path, json).unwrap();
                println!("Registro de horas inserido com sucesso.");
            }
        },
        Command::Read(r) => {
            let mut path = exe_path.clone();
            path.push(format!("{}.json", r.usuario));
        
            if !Path::new(&path).exists() {
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
    }
}
