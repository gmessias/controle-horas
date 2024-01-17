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

    match opts.subcmd {
        Command::Create(c) => command_create_user(&exe_path, &c.usuario),
        Command::Insert(c) => command_insert_time_recording(&exe_path, &c),
        Command::Read(r) => command_read_json(&exe_path, &r),
    }
}
