use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct Tailwind {
    path: String,
    template_path: String,
}

impl WritableTemplate for Tailwind {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl Tailwind {
    pub fn new() -> Self {
        Tailwind {
            path: Dir::Root(Some("tailwind.config.js")).path(),
            template_path: "tailwind_config.template".to_string(),
        }
    }
}