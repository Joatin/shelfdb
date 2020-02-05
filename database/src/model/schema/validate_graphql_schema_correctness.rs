use graphql_parser::schema::{Document, Definition, TypeDefinition, Directive, Field, Type};
use failure::Error;
use slog::Logger;
use colored::*;

pub const RESERVED_TYPE_NAMES: &[&str] = &["Query", "Mutation"];
pub const COLLECTION_DIRECTIVE_NAME: &str = "collection";
pub const KNOWN_DIRECTIVES: &[&str] = &[COLLECTION_DIRECTIVE_NAME];

pub fn validate_graphql_schema_correctness(logger: &Logger, document: &Document) -> Result<(), Error> {
    debug!(logger, "Validating schema definition");
    let mut schema_objects = vec![];
    for raw_def in &document.definitions {
        match raw_def {
            Definition::SchemaDefinition(def) => {
                crit!(logger, "You should not write a schema type in the definition. You should only write type definitions"; "position" => format!("{}", def.position));
                bail!("Schema definitions are not allowed in definition");
            },
            Definition::TypeDefinition(def) => {
                match def {
                    TypeDefinition::Scalar(_) => {},
                    TypeDefinition::Object(o) => {
                        if RESERVED_TYPE_NAMES.iter().find(|i| i.to_lowercase() == o.name.to_lowercase()).is_some() {
                            crit!(logger, "The type name \"{}\" is reserved", o.name; "position" => format!("{}", o.position));
                            bail!("The type name \"{}\" is reserved", o.name);
                        }
                        for directive in &o.directives {
                            if is_unknown_directives(directive) {
                                warn!(logger, "Found unknown directive \"{}\" for type \"{}\"", directive.name, o.name; "position" => format!("{}", directive.position));
                            }
                        }
                        if has_collection_directive(&o.directives) {
                            // This is a collection, let's make sure it has an Id
                            if !has_id(&o.fields) {
                                crit!(logger, "The collection \"{}\" does not have a valid id, the collection must have a fields that equal \"{}\"", o.name, "id: Uuid!".magenta());
                                bail!("The collection \"{}\" does not have a valid id", o.name);
                            }

                            schema_objects.push(o.clone());
                        } else {
                            warn!(logger, "This schema definitions does not contain any definitions! 🤷‍ Remember to add this line to the top of your schema \"{}\"", "directive @collection on OBJECT".magenta())
                        }
                    },
                    TypeDefinition::Interface(_) => {},
                    TypeDefinition::Union(_) => {},
                    TypeDefinition::Enum(_) => {},
                    TypeDefinition::InputObject(o) => {
                        crit!(logger, "Input objects are not allowed"; "position" => format!("{}", o.position));
                        bail!("Input objects are not allowed")
                    },
                }
            },
            Definition::TypeExtension(def) => {

            },
            Definition::DirectiveDefinition(def) => {

            },
        }
    }

    debug!(logger, "Schema is looking good! 👍");
    Ok(())
}

fn has_collection_directive(directives: &Vec<Directive>) -> bool {
    directives.iter().find(|i| i.name == COLLECTION_DIRECTIVE_NAME.to_string()).is_some()
}

fn is_unknown_directives(directive: &Directive) -> bool {
    !KNOWN_DIRECTIVES.contains(&&*directive.name)
}

fn has_id(fields: &Vec<Field>) -> bool {
    fields.iter().find(|i| i.name == "id".to_string() && i.field_type == Type::NonNullType(Box::new(Type::NamedType("Uuid".to_string())))).is_some()
}

#[cfg(test)]
mod test {
    use graphql_parser::parse_schema;
    use crate::model::schema::validate_graphql_schema_correctness::{validate_graphql_schema_correctness, RESERVED_TYPE_NAMES};
    use sloggers::null::NullLoggerBuilder;
    use sloggers::Build;

    #[test]
    fn it_throw_if_provided_schema_with_schema_definition() {
        let logger = NullLoggerBuilder.build().unwrap();

        let schema = r#"
            schema {
              query: TestQuery
            }

            type TestQuery {
                id: Uuid!
            }
        "#;

        let document = parse_schema(&schema).unwrap();

        assert_eq!(format!("{}", validate_graphql_schema_correctness(&logger, &document).unwrap_err()), "Schema definitions are not allowed in definition", "It should throw when the schema include a schema definition");
    }

    #[test]
    fn it_throw_if_provided_with_a_input_object() {
        let logger = NullLoggerBuilder.build().unwrap();

        let schema = r#"
            input TestInput {
                test: String
            }
        "#;

        let document = parse_schema(&schema).unwrap();

        assert_eq!(format!("{}", validate_graphql_schema_correctness(&logger, &document).unwrap_err()), "Input objects are not allowed", "It should throw when the schema includes a input object");
    }

    #[test]
    fn it_throw_if_a_type_has_a_reserved_name() {
        let logger = NullLoggerBuilder.build().unwrap();

        for reserved_name in RESERVED_TYPE_NAMES {
            for name in &[reserved_name.to_string(), reserved_name.to_lowercase(), reserved_name.to_uppercase()] {
                let schema = format!(r#"
                    type {} {{
                        id: Uuid!
                    }}
                "#, name);

                let document = parse_schema(&schema).unwrap();

                assert!(validate_graphql_schema_correctness(&logger, &document).is_err(), format!("It should throw when the schema includes a type with name {} since it's reserved", name));
                assert_eq!(format!("{}", validate_graphql_schema_correctness(&logger, &document).unwrap_err()), format!("The type name \"{}\" is reserved", name), "It should throw when the schema includes a type with name {} since it's reserved", name);
            }
        }
    }

    #[test]
    fn it_throw_if_a_collection_does_not_have_an_id() {
        let logger = NullLoggerBuilder.build().unwrap();

        let schema = r#"
            directive @collection on OBJECT

            type Collection @collection {
                test: String
            }
        "#;

        let document = parse_schema(&schema).unwrap();

        assert_eq!(format!("{}", validate_graphql_schema_correctness(&logger, &document).unwrap_err()), "The collection \"Collection\" does not have a valid id", "It should throw when a collection does not have an id");
    }

    #[test]
    fn it_not_throw_on_valid_schema() {
        let logger = NullLoggerBuilder.build().unwrap();

        let schema = r#"
            directive @collection on OBJECT

            type Collection @collection {
                id: Uuid!
            }
        "#;

        let document = parse_schema(&schema).unwrap();

        assert!(validate_graphql_schema_correctness(&logger, &document).is_ok(), "It should not throw");
    }
}