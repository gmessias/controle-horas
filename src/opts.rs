use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Controle de Horas CLI")]
#[command(author = "Gabriel Messias https://github.com/GMessias")]
#[command(version = "0.1.0")]
#[command(help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}")]
#[command(about, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(about = "Cria um usuário e vincula os próximos comandos a ele.")]
    Create(Create),
    #[clap(about = "Insere um novo registro.")]
    Insert(Insert),
    #[clap(about = "Lê os registros de acordo com o filtro informado.")]
    Read(Read),
}

#[derive(Parser, Debug)]
pub struct Create {
    #[clap(short = 'u', long = "user")]
    pub user: String,
}

#[derive(Parser, Debug)]
pub struct Insert {
    #[clap(short = 'i', long = "id")]
    pub id: String,
    #[clap(short = 'd', long = "data")]
    pub date: String,
    #[clap(short = 'H', long = "horas")]
    pub hours: Option<u8>,
    #[clap(short = 'm', long = "minutos")]
    pub minutes: Option<u8>,
    #[clap(short = 't', long = "tempo")]
    pub time: Option<String>,

}

#[derive(Parser, Debug)]
pub struct Read {
    #[clap(short = 'I', long = "id")]
    pub id: Option<String>,
    #[clap(short = 'D', long = "dia")]
    pub day: Option<String>,
    #[clap(short = 'M', long = "mes")]
    pub month: Option<String>,
}
