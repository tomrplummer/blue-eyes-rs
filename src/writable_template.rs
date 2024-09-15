use std::fs::File;
use std::io::Write;
use std::str::from_utf8;
use rust_embed::RustEmbed;
use tera::{Context, Tera};

#[derive(RustEmbed)]
#[folder = "file_templates"]
struct FileTemplates;

pub trait WritableTemplate {
    fn path(&self) -> String;
    fn template_path(&self) -> String;

    fn render(&self, filename: &str, context: &Context) -> Result<String, String> {
        let mut tera = Tera::default();

        if let Some(template_data) = FileTemplates::get(filename) {
            let template_str = from_utf8(template_data.data.as_ref())
                .expect("Failed to convert template to string");

            tera.add_raw_template(filename, template_str)
                .expect("Failed to add template to Tera");
        } else {
            println!("Template not found");
            return Err("Template not found".to_string());
        }
        match tera.render(filename, context) {
            Ok(rendered) => Ok(rendered),
            Err(e) => Err(e.to_string()),
        }
    }
    fn write_template(&mut self, context: &Context) -> Result<bool, String> {
        let file_contents = match self.render(&self.template_path(), context) {
            Ok(file_contents) => file_contents,
            Err(_) => return Err("Unable to generate template".to_string())
        };

        match self.write_to_file(&file_contents) {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }

    fn write_to_file(&self, content: &str) -> Result<(), String> {
        let mut file = match File::create(self.path()) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}