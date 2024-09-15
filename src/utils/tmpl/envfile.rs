use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct EnvFile {
    pub path: String,
    pub template_path: String,
    pub connection_string: String,
    pub secret: String,
}

impl WritableTemplate for EnvFile {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl EnvFile {
    pub fn new(project_name: String, db: String) -> Self {
        let pg_connection_string = "postgres://".to_string() + project_name.as_str();
        let sqlite_connection_string = "sqlite://".to_string() + project_name.as_str() + ".db";
        let connection_string = match db {
            _ if db.to_string().trim() == "postgres" => pg_connection_string,
            _ => sqlite_connection_string,
        };

        let secret = String::from("my_secret");
        let path = Dir::Root(Some(".env")).path();

        EnvFile {
            path,
            template_path: String::from("env_file.template"),
            connection_string,
            secret,
        }
    }
}

