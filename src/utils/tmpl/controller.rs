use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct Controller {
    path: String,
    template_path: String,
}

impl WritableTemplate for Controller {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl Controller {
    pub fn new(path: String) -> Self {
        Controller {
            path: Dir::Controllers(Some(&format!("{}{}", path, "_controller.rb"))).path(),
            template_path: "controller.template".to_string(),
        }
    }
}