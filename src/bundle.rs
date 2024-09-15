use std::process::Command;
use std::vec;

pub struct Bundler<'a> {
    pub  gems: Vec<&'a str>
}

impl<'a> Bundler<'a> {
    pub fn new() -> Self {
        let gems = vec![
            "sinatra",
            "sinatra-contrib",
            "sinatra-flash",
            "sequel",
            "rackup",
            "haml",
            "puma",
            "activesupport",
            "bcrypt",
            "jwt",
            "dotenv",
            "toml-rb",
            "foreman"
        ];

        Bundler { gems }
    }

    pub fn install(&self, db: String) -> Result<(), String> {
        let db_driver = if db == "postgres" {
            "pg"
        } else {
            "sqlite3"
        };

        let mut gem_list = self.gems.clone();
        gem_list.push(db_driver);

        for gem in gem_list {
            println!("Installing {}", gem);

            let output = Command::new("bundle")
                .arg("add")
                .arg(gem)
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                return Err(String::from_utf8(output.stderr).unwrap());
            }
        }

        Ok(())
    }
}
