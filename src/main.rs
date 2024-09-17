use clap::{Parser};
use colored::Colorize;
use env_logger::Env;

mod dirs;
mod utils;
mod writable_template;
mod bundle;
mod cli_commands;

use cli_commands::project::{Project};
use cli_commands::cli::GenerateSubcommand;
use crate::cli_commands::cli::{Cli, CommandType, Commands};
use crate::cli_commands::resource::Resource;
use crate::utils::tmpl::gemfile::Gemfile;



fn handle_generate(entity: &GenerateSubcommand) {
    let resource = match entity {
        GenerateSubcommand::Api(args) => Resource::new(args, CommandType::Api),
        GenerateSubcommand::Controller(args) => Resource::new(args, CommandType::Controller),
        GenerateSubcommand::Model(args) => Resource::new(args, CommandType::Model),
        GenerateSubcommand::Scaffold(args) => Resource::new(args, CommandType::Scaffold),
    };

    match resource.generate_template() {
        Ok(()) => println!("no error"),
        Err(e) => println!("{}", e),
    }
}

fn handle_new(project_name: String, db: String) -> Result<(), String> {
    let mut project = Project::new(project_name, db);
    if let Err(e) = project.generate() {
        println!("Failed to generate project {}", &e.red());
        return Err(e.to_string());
    }

    Ok(())
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::New { project_name, db } => {
            _ = handle_new(String::from(project_name), String::from(db))
        }
        Commands::Migrate => println!("Running all new migrations scripts"),
        Commands::Generate { entity } => handle_generate(entity),
        Commands::Test => {
            let gem_file = Gemfile::new();
            match gem_file.get_ruby_version() {
                Ok(ruby_version) => println!("{}", ruby_version),
                Err(e) => println!("{}", e),
            }
        }
    }
}
