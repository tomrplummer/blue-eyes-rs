use crate::cli_commands::cli::{CommandType, SharedArgs};

#[allow(dead_code)]
#[derive(Debug)]
struct Field {
    name: String,
    sql_type: String,
}

fn parse_fields(fields: Option<Vec<String>>) -> Option<Vec<Field>> {
    let field_list = match fields {
        Some(f) => f,
        _ => return None,
    };

    let mut new_fields = Vec::new();
    for field in field_list {
        let parts: Vec<&str> = field.split(":").collect();

        new_fields.push(Field {
            name: String::from(parts[1]),
            sql_type: String::from(parts[0]),
        });
    }

    Some(new_fields)
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Resource {
    name: String,
    fields: Option<Vec<Field>>,

    #[allow(dead_code)]
    alias: Option<String>,

    belongs_to: Option<String>,
        for_command: CommandType,
    }

impl Resource {
   pub fn new(args: &SharedArgs, cmd_type: CommandType) -> Self {
        Resource {
            name: args.name.clone(),
            fields: parse_fields(args.fields.clone()),
            alias: args.alias.clone(),
            belongs_to: args.belongs_to.clone(),
            for_command: cmd_type,
        }
    }
}