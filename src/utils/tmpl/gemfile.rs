use std::process::Command;
use tera::Context;
use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct Gemfile {
    path: String,
    template_path: String,
}

impl WritableTemplate for Gemfile {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl Gemfile {
    pub fn new() -> Self {
        Gemfile {
            path: Dir::Root(Some("Gemfile")).path(),
            template_path: "gemfile.template".to_string(),
        }
    }

    pub fn get_context(&self) -> Context {
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
}