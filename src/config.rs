use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.txt";

pub fn config_file_exists(exe_path: &PathBuf) -> bool {
    let mut path = exe_path.clone();
    let path = path.join(CONFIG_FILE_NAME);

    fs::metadata(&path).is_ok()
}

pub fn read_config_file(exe_path: &PathBuf) -> io::Result<String> {
    let mut path = exe_path.clone();
    path = path.join(CONFIG_FILE_NAME);

    if config_file_exists(&exe_path){
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let first_line = reader.lines().next();
        match first_line {
            Some(line) => Ok(line?),
            None => Err(io::Error::new(io::ErrorKind::Other, "Arquivo vazio")),
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Arquivo de configuração não encontrado"))
    }
}

pub fn create_config_file(exe_path: &PathBuf, username: &str) -> io::Result<()> {
    let mut path = exe_path.clone();
    path = path.join(CONFIG_FILE_NAME);

    let mut file = File::create(&path)?;
    file.write_all(username.as_bytes())?;

    Ok(())
}
