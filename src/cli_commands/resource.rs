use crate::cli_commands::cli::{CommandType, SharedArgs};
use crate::dirs::Dir;
use inflector::Inflector;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use chrono::Utc;
use tera::Context;
use toml::Value;
use crate::template_writer::write_template;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
struct Field {
    name: String,
    sql_type: String,
}

#[allow(dead_code)]
pub enum NameVariant {
    Model,
    Class,
    Variable,
    VariablePlural,
    Haml,
    Path,
    Alias,
    BelongsToModel,
    BelongsToPath,
    BelongsToId,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Resource {
    pub name: String,
    fields: Option<Vec<Field>>,

    #[allow(dead_code)]
    alias: Option<String>,

    belongs_to: Option<String>,
    for_command: CommandType,
}

impl Resource {
    pub fn new(args: &SharedArgs, cmd_type: CommandType) -> Self {
        let field_list = match &args.fields {
            Some(f) => {
                let mut new_fields = Vec::new();
                for field in f {
                    let parts: Vec<&str> = field.split(":").collect();

                    new_fields.push(Field {
                        name: String::from(parts[1]),
                        sql_type: String::from(parts[0]),
                    });
                }
                Some(new_fields)
            }
            _ => None,
        };

        Resource {
            name: args.name.clone(),
            fields: field_list,
            alias: args.alias.clone(),
            belongs_to: args.belongs_to.clone(),
            for_command: cmd_type,
        }
    }

    pub fn generate_template(&self) -> Result<(), String> {
        println!("Generating template... {:?}", &self.for_command);
        match &self.for_command {
            // CommandType::Api => println!("Api"),
            CommandType::Controller => {
                self.generate_controller()?;
                _ = self.generate_path_config();
                Ok(())
            }
            CommandType::Model => {
                println!("Generating model...");
                self.generate_model()?;
                println!("Generating migration...");
                self.generate_migration()?;
                Ok(())
            }
            // CommandType::Scaffold => println!("Scaffold"),
            _ => Err("Not implemented".to_string()),
        }
    }

    fn generate_model(&self) -> Result<(), String> {
        let filename = self.variant(NameVariant::Path, self.name.clone()) + ".rb";
        let context = self.get_context()?;
        let output_path = Dir::Models(Some(&filename)).path();

        match write_template(output_path, "model.template".to_string(), &context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn generate_migration(&self) -> Result<(), String> {
        let timestamp = Utc::now().timestamp();
        let filename = timestamp.to_string() + "_create_" + &self.variant(NameVariant::Path, self.name.clone()) + ".rb";
        let output_path = Dir::Migrations(Some(filename.as_str())).path();

        let context = self.get_migration_context()?;


        match write_template(output_path, "new_table.template".to_string(), &context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_migration_context(&self) -> Result<Context, String> {
        let mut context = Context::new();
        let table_name = self.variant(NameVariant::Path, self.name.clone());

        context.insert("table_name", &table_name);

        if let Some(belongs_to) = &self.belongs_to {
            let belongs_to_id = self.variant(NameVariant::BelongsToId, belongs_to.clone()) + "_id";
            context.insert("belongs_to_id", &belongs_to_id);
        }
        if let Some(fields) = &self.fields {
            context.insert("fields", fields);
        } else {
            return Err("No fields provided".to_string());
        }

        Ok(context)
    }
    fn get_context(&self) -> Result<Context, String> {
        let alias_lookup = match self.load_paths_config() {
            Ok(alias_lookup) => alias_lookup,
            Err(e) => Err(e.to_string())?,
        };

        let mut context = Context::new();
        context.insert("haml", &self.variant(NameVariant::Haml, self.name.clone()));
        context.insert(
            "variable",
            &self.variant(NameVariant::Variable, self.name.clone()),
        );
        context.insert(
            "variable_plural",
            &self.variant(NameVariant::VariablePlural, self.name.clone()),
        );
        context.insert(
            "class",
            &self.variant(NameVariant::Class, self.name.clone()),
        );
        context.insert(
            "model",
            &self.variant(NameVariant::Model, self.name.clone()),
        );
        if let Some(alias) = &self.alias {
            context.insert(
                "alias_or_name",
                &self.variant(NameVariant::Alias, alias.clone()),
            );
        } else {
            context.insert(
                "alias_or_name",
                &self.variant(NameVariant::Alias, self.name.clone()),
            );
        }

        if let Some(belongs_to) = &self.belongs_to {
            context.insert(
                "belongs_to_model",
                &self.variant(NameVariant::BelongsToModel, belongs_to.clone()),
            );
            context.insert(
                "belongs_to_id",
                &self.variant(NameVariant::BelongsToId, belongs_to.clone()),
            );
            context.insert(
                "belongs_to_path",
                &self.variant(
                    NameVariant::BelongsToPath,
                    alias_lookup
                        .get(&self.variant(NameVariant::BelongsToPath, belongs_to.to_string()))
                        .unwrap()
                        .clone(),
                ),
            );
        } else {
            context.insert(
                "belongs_to_model",
                &self.variant(NameVariant::BelongsToModel, self.name.clone()),
            );
            context.insert(
                "belongs_to_path",
                &self.variant(NameVariant::BelongsToPath, self.name.clone()),
            );
            context.insert(
                "belongs_to_id",
                &self.variant(NameVariant::BelongsToId, self.name.clone()),
            );
        }

        Ok(context)
    }

    fn get_path_config_context(
        &self,
        name: String,
        alias: Option<String>,
        belongs_to: Option<String>,
    ) -> Context {
        let mut context = Context::new();
        context.insert("name", &self.variant(NameVariant::Path, name));

        if let Some(alias) = alias {
            context.insert("alias", &self.variant(NameVariant::Path, alias));
            context.insert("has_alias", &true);
        } else {
            context.insert("alias", "");
            context.insert("has_alias", &false);
        }
        if let Some(belongs_to) = belongs_to {
            context.insert("belongs_to", &self.variant(NameVariant::Path, belongs_to));
            context.insert("has_belongs_to", &true);
        } else {
            context.insert("belongs_to", "");
            context.insert("has_belongs_to", &false);
        }

        context
    }

    fn load_paths_config(&self) -> Result<HashMap<String, String>, String> {
        let content = fs::read_to_string(Dir::Helpers(Some("paths_config.toml")).path())
            .unwrap_or_else(|e| e.to_string());

        let parsed: Value = match content.parse::<Value>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        let mut hash: HashMap<String, String> = HashMap::new();

        if let Some(resources) = parsed.get("resources").and_then(|v| v.as_array()) {
            for i in resources {
                let name = match i.get("name").and_then(|v| v.as_str()) {
                    Some(n) => n,
                    None => Err("Resource does not have a name")?,
                };
                if let Some(alias) = i.get("as").and_then(|v| v.as_str()) {
                    if alias.is_empty() {
                        hash.insert(name.to_string(), name.to_string());
                    } else {
                        hash.insert(name.to_string(), alias.to_string());
                    }
                } else {
                    hash.insert(name.to_string(), name.to_string());
                }
            }
        }

        Ok(hash)
    }

    fn generate_path_config(&self) -> Result<(), String> {
        let context = self.get_path_config_context(
            self.name.clone(),
            self.alias.clone(),
            self.belongs_to.clone(),
        );

        let output_path = Dir::Helpers(Some("paths_config.toml")).path();
        let template_path = "paths_config.template".to_string();

        match write_template(output_path, template_path, &context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn generate_controller(&self) -> Result<(), String> {
        let filename = self.variant(NameVariant::Path, self.name.clone()) + ".rb";
        let has_belongs_to = self.belongs_to.is_some();
        let output_path = Dir::Controllers(Some(&filename)).path();

        let context = &self.get_context()?;
        let template_path = if has_belongs_to {
            "controller_belongs_to.template"
        } else {
            "controller.template"
        };

        _ = match write_template(output_path, template_path.to_string(), context) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        let controller_name = self.variant(NameVariant::Class, self.name.clone()) + "Controller";
        match self.update_config_ru(controller_name) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn update_config_ru(&self, controller_name: String) -> Result<(), String> {
        let contents = match fs::read_to_string(Dir::Root(Some("config.ru")).path()) {
            Ok(contents) => contents,
            Err(e) => return Err(e.to_string()),
        };

        let replacement = "use ".to_string() + &controller_name + "\nrun Sinatra::Application";

        let result = contents.replace("run Sinatra::Application", &replacement);
        match fs::write(Dir::Root(Some("config.ru")).path(), result) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn variant(&self, variant: NameVariant, name: String) -> String {
        match variant {
            NameVariant::Model => name.to_pascal_case().to_singular(),
            NameVariant::Class => name.to_pascal_case().to_plural(),
            NameVariant::Variable => name.to_snake_case().to_singular(),
            NameVariant::Haml => name.to_snake_case().to_plural(),
            NameVariant::Path => name.to_snake_case().to_plural(),
            NameVariant::Alias => name.to_snake_case().to_plural(),
            NameVariant::BelongsToModel => name.to_pascal_case().to_singular(),
            NameVariant::BelongsToPath => name.to_snake_case().to_plural(),
            NameVariant::BelongsToId => name.to_snake_case().to_singular(),
            NameVariant::VariablePlural => name.to_snake_case().to_plural(),
        }
    }
}
