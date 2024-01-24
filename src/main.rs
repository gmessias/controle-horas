mod config;
mod opts;
mod usuario;
mod registro_horas;
mod command_create;
mod command_insert;
mod command_read;

use clap::Parser;
use std::env;
use opts::{Opts, Command};
use command_create::command_create_user;
use command_insert::command_insert_time_recording;
use command_read::command_read_json;

fn main() {
    let opts: Opts = Opts::parse();

    let mut exe_path = env::current_exe().unwrap();
    exe_path.pop();

    //let username = config::get_current_user(&exe_path).expect("Unable to read username");

    match opts.subcmd {
        Command::Create(cc) => {
            config::create_config_file(&exe_path, &cc.usuario).expect("Unable to set current user");
            command_create_user(&exe_path, &cc.usuario);
        },
        Command::Insert(ci) => command_insert_time_recording(&exe_path, &ci),
        Command::Read(cr) => command_read_json(&exe_path, &cr),
    }
}
