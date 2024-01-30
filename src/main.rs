use clap::Parser;
use command_create::command_create_user;
use command_insert::command_insert_time_recording;
use command_read::command_read_json;
use opts::{Command, Opts};
use std::env;

mod config;
mod opts;
mod user;
mod time_record;
mod command_create;
mod command_insert;
mod command_read;

fn main() {
    let opts: Opts = Opts::parse();

    let mut exe_path = env::current_exe().unwrap();
    exe_path.pop();

    match opts.subcmd {
        Command::Create(cc) => {
            config::create_config_file(&exe_path, &cc.user).expect("Não foi possível definir o usuário atual");
            command_create_user(&exe_path, &cc.user);
        }
        Command::Insert(ci) => command_insert_time_recording(&exe_path, &ci),
        Command::Read(cr) => command_read_json(&exe_path, &cr),
    }
}
