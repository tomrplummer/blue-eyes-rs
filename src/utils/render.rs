// use std::str::from_utf8;
// use rust_embed::RustEmbed;
// use tera::{Context, Tera};
//
// #[derive(RustEmbed)]
// #[folder = "file_templates"]
// struct FileTemplates;
// pub fn render(filename: &str, context: &Context) -> Result<String, String> {
//     let mut tera = Tera::default();
//     if let Some(template_data) = FileTemplates::get(filename) {
//         let template_str = from_utf8(template_data.data.as_ref())
//             .expect("Failed to convert template to string");
//
//         tera.add_raw_template(filename, template_str)
//             .expect("Failed to add template to Tera");
//     } else {
//         println!("Template not found");
//         return Err("Template not found".to_string());
//     }
//     match tera.render(filename, &context) {
//         Ok(rendered) => Ok(rendered),
//         Err(e) => Err(e.to_string()),
//     }
// }