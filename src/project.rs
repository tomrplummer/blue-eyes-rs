use rust_embed::RustEmbed;
use std::env::{self, current_dir};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tera::Context;
use crate::utils::tmpl::envfile::EnvFile;
use crate::dirs::Dir;
use crate::bundle::Bundler;
use crate::utils::tmpl::bundle_config::BundleConfig;
use crate::utils::tmpl::config_ru::ConfigRu;
use crate::utils::tmpl::gemfile::Gemfile;
use crate::writable_template::WritableTemplate;

#[derive(RustEmbed)]
#[folder = "project_template"]
struct Templates;
#[derive(Clone)]
pub struct Project {
    name: String,
    db: String,
}

impl Project {
    pub fn new(name: String, db: String) -> Self {
        Project { name, db }
    }

    pub fn generate(&self) -> Result<(), String> {
        let current_dir = current_dir().unwrap();
        let template_dir = current_dir.join(self.name.clone());

        _ = self.copy_project_template(template_dir);

        if let Err(e) = self.cd_app_dir() {
            return Err(e.to_string());
        }

        if let Err(e) = self.create_env_file() {
            return Err(e.to_string());
        }

        if let Err(e) = self.create_bundle_config() {
            return Err(e.to_string());
        }

        if let Err(e) = self.create_gemfile() {
            return Err(e.to_string());
        }

        if let Err(e) = self.create_config_ru() {
            return Err(e.to_string());
        }

        if let Err(e) = self.run_bundle(self.db.clone()) {
            return Err(e.to_string());
        }

        if let Err(e) = self.chmod_x(Dir::Bin(Some("dev")).path()) {
            return Err(e.to_string());
        }

        Ok(())
    }

    fn chmod_x(&self, path: String) -> Result<(), String> {
        let output = Command::new("chmod").arg("+x").arg(path).output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }

        Ok(())
    }

    fn run_bundle(&self, db: String) -> Result<(), String> {
        let bundler = Bundler::new();
        match bundler.install(db) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn create_config_ru(&self) -> Result<(), String> {
        let mut config_ru = ConfigRu::new();
        match config_ru.write_template(&Context::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn cd_app_dir(&self) -> Result<(), String> {
        let app_path = Dir::Root(Some(&self.name)).path();
        let app_dir = Path::new(&app_path);

        if let Err(e) = env::set_current_dir(app_dir) {
            return Err(e.to_string());
        }

        Ok(())
    }

    fn create_env_file(&self) -> Result<(), String> {
        let mut env = EnvFile::new(self.name.clone(), self.db.clone());

        let mut context = Context::new();
        context.insert("connection_string", &env.connection_string);
        context.insert("secret", &env.secret);

        match env.write_template(&context) {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn create_bundle_config(&self) -> Result<(), String> {
        let mut bundle_config = BundleConfig::new();

        match bundle_config.write_template(&Context::new()) {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn create_gemfile(&self) -> Result<(), String> {
        let mut gemfile = Gemfile::new();
        let context = gemfile.get_context();

        match gemfile.write_template(&context) {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e.to_string()),
        }
    }

    fn copy_project_template(&self, template_dir: PathBuf) -> Result<(), String> {
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
