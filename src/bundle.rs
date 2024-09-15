use std::process::{Command, Stdio};
use std::{thread, vec};
use std::io::{BufRead, BufReader};
use thread::spawn;
use colored::Colorize;

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
    pub fn install(&self, db: &str) -> Result<(), String> {
        let cmd = self.build_command(db);

        match self.run_install(cmd) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }

    fn build_command(&self, db: &str) -> Command {
        let mut cmd = Command::new("bundle");
        cmd.arg("add");
        for gem in &self.gems {
            cmd.arg(gem);
        }

        if db == "postgres" {
            cmd.arg("pg");
        } else {
            cmd.arg("sqlite3");
        }
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        cmd
    }

    pub fn run_install(&self, mut cmd: Command) -> Result<(), String> {
        let mut child = cmd.spawn().map_err(|err| err.to_string())?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            spawn(move || {
                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line.blue()),
                        Err(e)  => println!("Error: {}", e.to_string().red())
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
