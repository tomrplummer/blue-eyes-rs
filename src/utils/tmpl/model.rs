use crate::dirs::Dir;
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
        self.template_path.to_string()
    }
}

impl Model {
   pub fn new(name: String) -> Model {
       let filename = format!("{}{}", name, ".rb");
       
       Model {
           path: Dir::Models(Some(filename.as_str())).path(),
           template_path: "model.template".to_string(),
       }
   }
}