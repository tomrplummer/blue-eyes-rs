use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct PathsConfig {
    path: String,
    template_path: String,
}

impl WritableTemplate for PathsConfig {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl PathsConfig {
    pub fn new() -> PathsConfig {
        PathsConfig {
            path: Dir::Helpers(Some("paths_config.toml")).path(),
            template_path: "paths_config.template".to_string(),
        }
    }
}

