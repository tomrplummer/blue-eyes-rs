use clap::Parser;
use colored::Colorize;
use env_logger::Env;

mod bundle;
mod cli_commands;
mod dirs;
mod utils;
mod template_writer;
mod migrate;

use crate::cli_commands::cli::{Cli, CommandType, Commands};
use crate::cli_commands::resource::Resource;
use cli_commands::cli::GenerateSubcommand;
use cli_commands::project::Project;

fn handle_generate(entity: &GenerateSubcommand) -> Result<(), String> {
    let resource = match entity {
        GenerateSubcommand::Api(args) => Resource::new(args, CommandType::Api),
        GenerateSubcommand::Controller(args) => Resource::new(args, CommandType::Controller),
        GenerateSubcommand::Model(args) => Resource::new(args, CommandType::Model),
        GenerateSubcommand::Scaffold(args) => Resource::new(args, CommandType::Scaffold),
        //GenerateSubcommand::Migration{entity} => Res 
        _ => return Err("Not implemented".to_string())
    };

    match resource.generate_template() {
        Ok(_) => Ok(()),
        Err(e) => Err(e), 
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

    let result = match &cli.command {
        Commands::New { project_name, db } => {
            handle_new(String::from(project_name), String::from(db))
        }
        Commands::Migrate => migrate::run(),
        Commands::Generate { entity } => handle_generate(entity),
        Commands::G { entity } => handle_generate(entity),
    };

    match result {
        Ok(()) => println!("{}", "Complete".green().bold()),
        Err(e) => println!("Error: {}", e.red().bold())
    }
}
