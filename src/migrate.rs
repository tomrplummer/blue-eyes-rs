use std::env;
use std::process::Command;
use dotenvy::dotenv;
use crate::dirs::Dir;

pub fn run () -> Result<(), String> {
    dotenv().ok();
    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let cmd = Command::new("bundle")
        .arg("exec")
        .arg("sequel")
        .arg("-m")
        .arg(Dir::Migrations(None).path())
        .arg(connection_string.clone())
        .output()
        .map_err(|e| e.to_string())?;

    if !cmd.status.success() {
        return Err(String::from_utf8(cmd.stderr).unwrap());
    }

    Ok(())
}