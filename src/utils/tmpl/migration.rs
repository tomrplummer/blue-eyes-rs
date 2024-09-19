use chrono::Utc;
use crate::dirs::Dir;
use crate::writable_template::WritableTemplate;

pub struct Migration {
    path: String,
    template_path: String,
}

impl WritableTemplate for Migration {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn template_path(&self) -> String {
        self.template_path.to_string()
    }
}

impl Migration {
    pub fn new(name: String) -> Migration {
        let timestamp = Utc::now().timestamp();
        let filename = timestamp.to_string() + "_create_" + name.as_str() + ".rb";
        
        Migration {
            path: Dir::Migrations(Some(filename.as_str())).path(),
            template_path: "new_table.template".to_string()
        }
    }
}