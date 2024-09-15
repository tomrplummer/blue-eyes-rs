use std::process::{Command, Stdio};
use std::{thread, vec};
use std::io::{BufRead, BufReader};
use thread::spawn;

pub struct Bundler<'a> {
    pub gems: Vec<&'a str>
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
        let bundle = &mut Command::new("bundle".to_string());
        bundle.arg("add");
        for gem in &self.gems {
            bundle.arg(&gem);
        }

        if db == "postgres" {
            bundle.arg("pg");
        } else {
            bundle.arg("sqlite3");
        }
        bundle.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = bundle.spawn().map_err(|err| err.to_string())?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            spawn(move || {
                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(e)  => println!("Error: {}", e)
                    }
                }
            });
        };

        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            spawn(move || {
                for line in reader.lines() {
                    match line {
                        Ok(line) => eprintln!("{}", line),
                        Err(e)  => println!("Error: {}", e)
                    }
                }
            });
        };

        let status = child.wait().map_err(|err| err.to_string())?;

        if !status.success() {
            Err(status.code().unwrap_or(-1).to_string())
        } else {
            Ok(())
        }
    }
}
