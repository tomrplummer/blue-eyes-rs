use clap::{Args, Parser, Subcommand};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CommandType {
    New,
    Api,
    Controller,
    Model,
    Migrate,
    Scaffold,
}

#[derive(Parser, Debug)]
#[command(
    name = "blue-eyes",
    about = "A cli_commands to generate projects and files for Ruby, Sinatra and Haml/Tailwind"
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    New {
        project_name: String,

        #[arg(long, default_value = "sqlite")]
        db: String,
    },
    Test,
    Migrate,
    Generate {
        #[command(subcommand)]
        entity: GenerateSubcommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubcommand {
    Controller(SharedArgs),
    Model(SharedArgs),
    Api(SharedArgs),
    Scaffold(SharedArgs),
}

#[derive(Args, Debug)]
pub struct SharedArgs {
    pub name: String,

    #[arg(long, value_delimiter = ' ')]
    pub fields: Option<Vec<String>>,

    #[arg(long)]
    pub alias: Option<String>,

    #[arg(long)]
    pub belongs_to: Option<String>,
}
