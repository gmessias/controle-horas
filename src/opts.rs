use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Create(Create),
    Insert(Insert),
    Read(Read),
}

#[derive(Parser, Debug)]
pub struct Create {
    #[clap(short, long)]
    pub usuario: String,
}

#[derive(Parser, Debug)]
pub struct Insert {
    //#[clap(short, long)]
    //pub usuario: String,
    #[clap(long)]
    pub id: String,
    #[clap(long)]
    pub data: String,
    #[clap(long)]
    pub horas: u8,
}

#[derive(Parser, Debug)]
pub struct Read {
    //#[clap(short, long)]
    //pub usuario: String,
    #[clap(long)]
    pub id: Option<String>,
    #[clap(long)]
    pub dia: Option<String>,
    #[clap(long)]
    pub mes: Option<String>,
}
