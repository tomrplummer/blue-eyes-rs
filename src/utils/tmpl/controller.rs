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
    pub fn new(filename: String, belongs_to: bool) -> Self {
        let template_path = if belongs_to {
            "controller_belongs_to.template".to_string()
        } else {
            "controller.template".to_string()
        };

        Controller {
            path: Dir::Controllers(Some(&format!("{}{}", filename, "_controller.rb"))).path(),
            template_path,
        }
    }
}