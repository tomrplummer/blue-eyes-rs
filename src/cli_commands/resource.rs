use crate::cli_commands::cli::{CommandType, SharedArgs};
use inflector::Inflector;
use tera::Context;
use crate::utils::tmpl::controller::Controller;
use crate::writable_template::WritableTemplate;

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
        match &self.for_command {
            // CommandType::Api => println!("Api"),
            CommandType::Controller => self.generate_controller(),
            // CommandType::Model => println!("Model"),
            // CommandType::Scaffold => println!("Scaffold"),
            _ => Err("Not implemented".to_string()),
        }
    }

    fn get_context(&self) -> Context {
        let mut context = Context::new();
        context.insert("haml", &self.variant(NameVariant::Haml, self.name.clone()));
        context.insert("variable", &self.variant(NameVariant::Variable, self.name.clone()));
        context.insert("variable_plural", &self.variant(NameVariant::VariablePlural, self.name.clone()));
        context.insert("class", &self.variant(NameVariant::Class, self.name.clone()));
        context.insert("model", &self.variant(NameVariant::Model, self.name.clone()));
        if let Some(alias) = &self.alias {
            context.insert("alias_or_name", &self.variant(NameVariant::Alias, alias.clone()));
        } else {
            context.insert("alias_or_name", &self.variant(NameVariant::Alias, self.name.clone()));
        }

        context
    }

    fn generate_controller(&self) -> Result<(), String> {
        let name = &self.variant(NameVariant::Path, self.name.clone());
        let mut controller = Controller::new(name.clone());
        match controller.write_template(&self.get_context()) {
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
            NameVariant::Alias => {
                if let Some(name) = &self.alias {
                    name.to_snake_case().to_plural()
                } else {
                    self.name.to_snake_case().to_plural()
                }
            },
            NameVariant::BelongsToModel => {
                if let Some(belongs_to) = &self.belongs_to {
                    belongs_to.to_pascal_case().to_singular()
                } else {
                    self.name.to_snake_case().to_plural()
                }
            },
            NameVariant::BelongsToPath => {
                if let Some(belongs_to) = &self.belongs_to {
                    belongs_to.to_snake_case().to_plural()
                } else {
                    self.name.to_snake_case().to_plural()
                }
            },
            NameVariant::VariablePlural => name.to_snake_case().to_plural(),
        }
    }

    // pub fn name_variant(&self, variant: NameVariant) -> String {
    //     match variant {
    //         NameVariant::Model => self.name.to_pascal_case().to_singular(),
    //         NameVariant::Class => self.name.to_pascal_case().to_plural(),
    //         NameVariant::Variable => self.name.to_snake_case().to_singular(),
    //         NameVariant::Path => self.name.to_snake_case().to_plural(),
    //         NameVariant::Alias => {
    //             if let Some(alias) = &self.alias {
    //                 alias.to_snake_case().to_plural()
    //             } else {
    //                 self.name.to_snake_case().to_plural()
    //             }
    //         },
    //         _ => "unimplemented".to_string(),
    //     }
    // }
}
