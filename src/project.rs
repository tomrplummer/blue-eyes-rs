use rust_embed::RustEmbed;
use std::env::{self, current_dir};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use colored::Colorize;
use tera::Context;
use crate::utils::tmpl::envfile::EnvFile;
use crate::dirs::Dir;
use crate::bundle::Bundler;
use crate::utils::fget::download_file;
use crate::utils::tmpl::bundle_config::BundleConfig;
use crate::utils::tmpl::config_ru::ConfigRu;
use crate::utils::tmpl::gemfile::Gemfile;
use crate::utils::tmpl::tw::Tailwind;
use crate::writable_template::WritableTemplate;

#[derive(RustEmbed)]
#[folder = "project_template"]
struct Templates;
#[derive(Clone)]
pub struct Project {
    name: String,
    db: String,
    connection_string: Option<String>,
}

impl Project {
    pub fn new(name: String, db: String) -> Self {
        Project { name, db, connection_string: None}
    }

    pub fn generate(&mut self) -> Result<(), String> {
        let current_dir = current_dir().unwrap();
        let template_dir = current_dir.join(self.name.clone());

        _ = self.copy_project_template(template_dir);

        // copy all files/folders from project_template
        if let Err(e) = self.cd_app_dir() {
            return Err(e.to_string());
        }

        // create .env file with db
        // TODO: generate secret
        self.connection_string = match self.create_env_file() {
            Ok(connection_string) => Some(connection_string),
            Err(e) => {
                return Err(e.to_string())
            }
        };

        // add bundle config to control bundler settings
        if let Err(e) = self.create_bundle_config() {
            return Err(e.to_string());
        }

        // add empty gemfile, except for required fields
        if let Err(e) = self.create_gemfile() {
            return Err(e.to_string());
        }

        // base config.ru for template
        if let Err(e) = self.create_config_ru() {
            return Err(e.to_string());
        }

        // add default gems and install
        if let Err(e) = self.run_bundle(self.db.clone()) {
            return Err(e.to_string());
        }

        // bin/dev loses execute, add back
        if let Err(e) = self.chmod_x(Dir::Bin(Some("dev")).path()) {
            return Err(e.to_string());
        }

        // run initial migrate, required for user model
        if let Err(e) = self.run_migrate() {
            return Err(e.to_string());
        }

        // download tailwind
        // mac version currently
        if let Err(e) = self.download_tailwind() {
            return Err(e.to_string());
        }

        // run init, make executable
        if let Err(e) = self.init_tailwind() {
            return Err(e.to_string());
        }

        println!("{}", "Run app".blue());
        println!("-------------");
        println!("{}", format!("{}", format_args!("cd ./{}", &self.name)).blue());
        println!("{}", "bin/dev".blue());

        Ok(())
    }

    fn download_tailwind(&self) -> Result<(), String>{
        println!("{}", "Downloading tailwind".blue());
        let url = "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64";
        if let Err(e) = download_file(url, &Dir::Bin(Some("tailwindcss")).path()) {
            return Err(e.to_string());
        }

        if let Err(e) = self.chmod_x(Dir::Bin(Some("tailwindcss")).path()) {
            return Err(e.to_string())
        }

        Ok(())
    }

    fn init_tailwind(&self) -> Result<(), String>{
        let cmd = Command::new(Dir::Bin(Some("tailwindcss")).path()).arg("init").output().map_err(|e| e.to_string())?;

        if !cmd.status.success() {
            return Err("Failed to init tailwind.".to_string());
        }

        let mut tailwind = Tailwind::new();
        match tailwind.write_template(&Context::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }

    }

    fn run_migrate(&self) -> Result<(), String> {
        let msg: String = format!("{}", format_args!("Running migrations for {}", self.connection_string.clone().unwrap()));
        println!("{}", msg.green());

        let cmd = Command::new("bundle")
            .arg("exec")
            .arg("sequel")
            .arg("-m")
            .arg(Dir::Migrations(None).path())
            .arg(self.connection_string.clone().unwrap())
            .output()
            .map_err(|e| e.to_string())?;

        if !cmd.status.success() {
            return Err(String::from_utf8(cmd.stderr).unwrap());
        }

        Ok(())
    }

    fn chmod_x(&self, path: String) -> Result<(), String> {
        println!("{}", "Setting execute for ".green());
        println!("{}", path.green());

        let output = Command::new("chmod").arg("+x").arg(path).output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }

        Ok(())
    }

    fn run_bundle(&self, db: String) -> Result<(), String> {
        println!("{}", "Installing default Gems".green());

        let bundler = Bundler::new();
        match bundler.install(&db) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn create_config_ru(&self) -> Result<(), String> {
        println!("{}", "Creating config.ru".green());

        let mut config_ru = ConfigRu::new();
        match config_ru.write_template(&Context::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn cd_app_dir(&self) -> Result<(), String> {
        println!("{}", "Moving into app directory".green());

        let app_path = Dir::Root(Some(&self.name)).path();
        let app_dir = Path::new(&app_path);

        if let Err(e) = env::set_current_dir(app_dir) {
            return Err(e.to_string());
        }

        Ok(())
    }

    fn create_env_file(&self) -> Result<String, String> {
        println!("{}", "Creating .env file".green());

        let mut env = EnvFile::new(self.name.clone(), self.db.clone());

        let mut context = Context::new();
        context.insert("connection_string", &env.connection_string);
        context.insert("secret", &env.secret);

        match env.write_template(&context) {
            Ok(_) => Ok(env.connection_string),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn create_bundle_config(&self) -> Result<(), String> {
        println!("{}", "Creating bundle config".green());

        let mut bundle_config = BundleConfig::new();

        match bundle_config.write_template(&Context::new()) {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn create_gemfile(&self) -> Result<(), String> {
        println!("{}", "Creating Gemfile".green());

        let mut gemfile = Gemfile::new();
        let context = gemfile.get_context();

        match gemfile.write_template(&context) {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn copy_project_template(&self, template_dir: PathBuf) -> Result<(), String> {
        println!("{}", "Copying project template files".green());

        for file in Templates::iter() {
            if let Some(content) = Templates::get(&file) {
                let dest_path = template_dir.join(String::from(file));
                if let Some(parent) = dest_path.parent() {
                    create_dir_all(parent).unwrap();
                }
                let mut output_file = File::create(&dest_path).unwrap();
                match output_file.write_all(content.data.as_ref()) {
                    Ok(_) => continue,
                    Err(e) => return Err(e.to_string()),
                }
            }
        }

        Ok(())
    }
}
