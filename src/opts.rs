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
    #[clap(short, long = "user")]
    pub user: String,
}

#[derive(Parser, Debug)]
pub struct Insert {
    #[clap(long = "id")]
    pub id: String,
    #[clap(long = "data")]
    pub date: String,
    #[clap(long = "horas")]
    pub hours: Option<u8>,
    #[clap(long = "minutos")]
    pub minutes: Option<u8>,
}

#[derive(Parser, Debug)]
pub struct Read {
    #[clap(long = "id")]
    pub id: Option<String>,
    #[clap(long = "dia")]
    pub day: Option<String>,
    #[clap(long = "mes")]
    pub month: Option<String>,
}
