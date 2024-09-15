use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct ConfigRu {
    path: String,
    template_path: String,
}

impl WritableTemplate for ConfigRu {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl ConfigRu {
    pub fn new() -> Self {
        ConfigRu {
            path: Dir::Root(Some("config.ru")).path(),
            template_path: "config_ru.template".to_string(),
        }
    }
}