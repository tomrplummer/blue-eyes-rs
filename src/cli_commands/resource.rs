use std::collections::HashMap;
use std::fs;
use crate::cli_commands::cli::{CommandType, SharedArgs};
use crate::utils::tmpl::controller::Controller;
use crate::utils::tmpl::paths_config::PathsConfig;
use crate::writable_template::WritableTemplate;
use inflector::Inflector;
use tera::Context;
use toml::Value;
use crate::dirs::Dir;
use crate::utils::tmpl::model::Model;

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
                self.generate_model()?; 
                Ok(())
            }
            // CommandType::Scaffold => println!("Scaffold"),
            _ => Err("Not implemented".to_string()),
        }
    }

    fn generate_model(&self) -> Result<(), String> {
        let filename = self.variant(NameVariant::Path, self.name.clone());
        let context = self.get_context()?;

        let mut model = Model::new(filename);

        match model.write_template(&context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
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
                &self.variant(NameVariant::BelongsToPath, alias_lookup.get(belongs_to).unwrap().clone()),
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
        let content = fs::read_to_string(Dir::Helpers(Some("paths_config.toml")).path()).unwrap_or_else(|e| e.to_string());

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
        let path_config_context = &self.get_path_config_context(
            self.name.clone(),
            self.alias.clone(),
            self.belongs_to.clone(),
        );

        let mut path_config = PathsConfig::new();

        match path_config.write_template(path_config_context) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn generate_controller(&self) -> Result<(), String> {
        let filename = self.variant(NameVariant::Path, self.name.clone());
        let has_belongs_to = self.belongs_to.is_some();

        let mut controller = Controller::new(filename.clone(), has_belongs_to);
        let context = &self.get_context()?;
        match controller.write_template(context) {
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
