use crate::writable_template::WritableTemplate;

pub struct Model {
    path: String,
    template_path: String,
}

impl WritableTemplate for Model {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.path.to_string()
    }
}

impl Model {
    
}