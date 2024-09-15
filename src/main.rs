use core::str;
// use std::io::Write;

use clap::{Args, Parser, Subcommand};

mod dirs;
mod utils;
pub mod project;
mod writable_template;

// use utils::tmpl;
// use utils::paths::Dir;
use project::Project;

#[allow(dead_code)]
#[derive(Debug)]
enum CommandType {
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
    about = "A cli to generate projects and files for Ruby, Sinatra and Haml/Tailwind"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    New {
        project_name: String,

        #[arg(long, default_value = "sqlite")]
        db: String,
    },
    Migrate,
    Generate {
        #[command(subcommand)]
        entity: GenerateSubcommand,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateSubcommand {
    Controller(SharedArgs),
    Model(SharedArgs),
    Api(SharedArgs),
    Scaffold(SharedArgs),
}

#[derive(Args, Debug)]
struct SharedArgs {
    name: String,

    #[arg(long, value_delimiter = ' ')]
    fields: Option<Vec<String>>,

    #[arg(long)]
    alias: Option<String>,

    #[arg(long)]
    belongs_to: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Field {
    name: String,
    sql_type: String,
}

fn parse_fields(fields: Option<Vec<String>>) -> Option<Vec<Field>> {
    let field_list = match fields {
        Some(f) => f,
        _ => return None,
    };

    let mut new_fields = Vec::new();
    for field in field_list {
        let parts: Vec<&str> = field.split(":").collect();

        new_fields.push(Field {
            name: String::from(parts[1]),
            sql_type: String::from(parts[0]),
        });
    }

    Some(new_fields)
}
#[allow(dead_code)]
#[derive(Debug)]
struct Resource {
    name: String,
    fields: Option<Vec<Field>>,

    #[allow(dead_code)]
    alias: Option<String>,

    belongs_to: Option<String>,
    for_command: CommandType,
}

impl Resource {
    fn new(args: &SharedArgs, cmd_type: CommandType) -> Self {
        Resource {
            name: args.name.clone(),
            fields: parse_fields(args.fields.clone()),
            alias: args.alias.clone(),
            belongs_to: args.belongs_to.clone(),
            for_command: cmd_type,
        }
    }
}

fn handle_generate(entity: &GenerateSubcommand) {
    let _resource = match entity {
        GenerateSubcommand::Api(args) => Resource::new(args, CommandType::Api),
        GenerateSubcommand::Controller(args) => Resource::new(args, CommandType::Controller),
        GenerateSubcommand::Model(args) => Resource::new(args, CommandType::Model),
        GenerateSubcommand::Scaffold(args) => Resource::new(args, CommandType::Scaffold),
    };
}

fn handle_new(project_name: String, db: String) -> Result<(), String> {
    let project = Project::new(project_name, db);
    if let Err(e) = project.generate() {
        return Err(e.to_string());
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { project_name, db } => {
            _ = handle_new(String::from(project_name), String::from(db))
        }
        Commands::Migrate => println!("Running all new migrations scripts"),
        Commands::Generate { entity } => handle_generate(entity),
    }
}
