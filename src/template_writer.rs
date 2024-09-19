use rust_embed::RustEmbed;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::from_utf8;
use tera::{Context, Tera};

#[derive(RustEmbed)]
#[folder = "file_templates"]
struct FileTemplates;

pub fn write_template(output_path: String, template_name: String, context: &Context) -> Result<bool, String> {
    let file_contents = match render(template_name.as_str(), context) {
        Ok(file_contents) => file_contents,
        Err(e) => return Err(e.to_string()),
    };

    match write_to_file(output_path, &file_contents) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

fn render(filename: &str, context: &Context) -> Result<String, String> {
    let mut tera = Tera::default();

    if let Some(template_data) = FileTemplates::get(filename) {
        let template_str =
            from_utf8(template_data.data.as_ref()).expect("Failed to convert template to string");

        tera.add_raw_template(filename, template_str)
            .expect("Failed to add template to Tera");
    } else {
        return Err("Template not found".to_string());
    }

    match tera.render(filename, context) {
        Ok(rendered) => Ok(rendered),
        Err(e) => Err(e.to_string()),
    }
}

fn write_to_file(output_path: String, content: &str) -> Result<(), String> {
    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)
    {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
