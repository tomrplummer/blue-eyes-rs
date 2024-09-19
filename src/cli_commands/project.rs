use crate::bundle::Bundler;
use crate::dirs::Dir;
use crate::utils::fget::download_file;
use colored::Colorize;
use rust_embed::RustEmbed;
use std::env::{self, current_dir};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use rand::RngCore;
use rand::rngs::OsRng;
use tera::Context;
use crate::template_writer::write_template;
use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

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
        Project {
            name,
            db,
            connection_string: None,
        }
    }

    pub fn generate(&mut self) -> Result<(), String> {
        let current_dir = current_dir().unwrap();
        let template_dir = current_dir.join(self.name.clone());

        // copy all files/folders from project_template
        self.copy_project_template(template_dir)?;
        self.cd_app_dir()?;

        // create .env file with db
        // TODO: generate secret
        self.connection_string = match self.create_env_file() {
            Ok(connection_string) => Some(connection_string),
            Err(e) => return Err(e.to_string()),
        };

        // add bundle config to control bundler settings
        self.create_bundle_config()?;

        // add empty gemfile, except for required fields
        self.create_gemfile()?;

        // base config.ru for template
        self.create_config_ru()?;

        // add default gems and install
        self.run_bundle(self.db.clone())?;

        // bin/dev loses execute, add back
        self.chmod_x(Dir::Bin(Some("dev")).path())?;

        // run initial migrate, required for user model
        self.run_migrate()?;

        // download tailwind
        // mac version currently
        self.download_tailwind()?;

        // run init, make executable
        self.init_tailwind()?;

        println!("\n{}", "Run app".blue());
        println!("-------------");
        println!(
            "{}",
            format!("{}", format_args!("cd ./{}", &self.name)).blue()
        );
        println!("{}\n", "bin/dev".blue());

        Ok(())
    }

    fn download_tailwind(&self) -> Result<(), String> {
        println!("{}", "Downloading tailwind".blue());
        let url = "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64";
        if let Err(e) = download_file(url, &Dir::Bin(Some("tailwindcss")).path()) {
            return Err(e.to_string());
        }

        if let Err(e) = self.chmod_x(Dir::Bin(Some("tailwindcss")).path()) {
            return Err(e.to_string());
        }

        Ok(())
    }

    fn init_tailwind(&self) -> Result<(), String> {
        let cmd = Command::new(Dir::Bin(Some("tailwindcss")).path())
            .arg("init")
            .output()
            .map_err(|e| e.to_string())?;

        if !cmd.status.success() {
            return Err("Failed to init tailwind.".to_string());
        }

        let output_path = Dir::Root(Some("tailwind.config.js")).path();
        let template_path = "tailwind_config.template".to_string();

        match write_template(output_path, template_path, &Context::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn run_migrate(&self) -> Result<(), String> {
        print!("{}", "Running migrations for ".green());
        println!("{}", self.connection_string.clone().unwrap().green().bold());

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
        print!("{}", "Setting execute for ".green());
        println!("{}", path.green().bold());

        let output = Command::new("chmod")
            .arg("+x")
            .arg(path)
            .output()
            .map_err(|e| e.to_string())?;
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

        let output_path = Dir::Root(Some("config.ru")).path();
        let template_path = "config_ru.template".to_string();

        match write_template(output_path, template_path, &Context::new()) {
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

    fn generate_secret(&self) -> String {
        let mut key = [0u8; 32];  // 32 bytes = 256 bits
        OsRng.fill_bytes(&mut key);  // Fill with cryptographically secure random bytes

        // Encode the secret key as base64
        BASE64.encode(key)
    }

    fn create_env_file(&self) -> Result<String, String> {
        println!("{}", "Creating .env file".green());

        let output_path = Dir::Root(Some(".env")).path();
        let template_path = "env_file.template".to_string();

        let pg_connection_string = "postgres://".to_string() + self.name.as_str();
        let sqlite_connection_string = "sqlite://".to_string() + self.name.as_str() + ".db";
        let connection_string = match self.db {
            _ if self.db.to_string().trim() == "postgres" => pg_connection_string,
            _ => sqlite_connection_string,
        };

        let secret = self.generate_secret();

        let mut context = Context::new();
        context.insert("connection_string", connection_string.as_str());
        context.insert("secret", secret.as_str());

        match write_template(output_path, template_path, &context) {
            Ok(_) => Ok(connection_string),
            Err(e) => Err(e.to_string()),
        }
    }

    fn create_bundle_config(&self) -> Result<(), String> {
        println!("{}", "Creating bundle config".green());

        let output_path = Dir::BundleConfig(Some("config")).path();
        let template_path = "bundle_config.template".to_string();

        match write_template(output_path, template_path, &Context::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_gemfile_context(&self) -> Context {
        let ruby_version = match self.get_ruby_version() {
            Ok(ruby_version) => ruby_version,
            Err(error) => panic!("{}", error),
        };

        let mut context = Context::new();
        context.insert("ruby_version", ruby_version.as_str());

        context
    }
    pub fn get_ruby_version(&self) -> Result<String, String> {
        let output = Command::new("ruby")
            .arg("--version")
            .output()
            .map_err(|err| err.to_string())?;

        let full_version = match String::from_utf8(output.stdout) {
            Ok(v) => v,
            Err(err) => return Err(err.to_string()),
        };

        let parts = full_version.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 2 {
            Err("Unable to get ruby version".to_string())
        } else {
            Ok(parts[1].trim().to_string())
        }
    }

    fn create_gemfile(&self) -> Result<(), String> {
        println!("{}", "Creating Gemfile".green());

        let output_path = Dir::Root(Some("Gemfile")).path();
        let template_path = "gemfile.template".to_string();
        let context = self.get_gemfile_context();
        match write_template(output_path, template_path, &context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
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
