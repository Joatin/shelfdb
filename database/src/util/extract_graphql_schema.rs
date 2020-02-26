use crate::util::has_collection_directive;
use graphql_parser::schema::{
    Definition,
    Document,
    ObjectType,
    TypeDefinition,
};

pub struct ExtractedData {
    pub collections: Vec<ObjectType>,
    pub other_types: Vec<ObjectType>,
}

pub fn extract_graphql_schema(doc: &Document) -> ExtractedData {
    let mut collections = vec![];
    let mut other_types = vec![];

    for definition in &doc.definitions {
        if let Definition::TypeDefinition(type_def) = definition {
            match type_def {
                TypeDefinition::Scalar(_) => { /* IGNORED */ }
                TypeDefinition::Object(o) => {
                    if has_collection_directive(&o.directives) {
                        collections.push(o.clone())
                    } else {
                        other_types.push(o.clone());
                    }
                }
                TypeDefinition::Interface(_) => { /* TODO */ }
                TypeDefinition::Union(_) => { /* TODO */ }
                TypeDefinition::Enum(_) => { /* TODO */ }
                TypeDefinition::InputObject(_) => { /* IGNORED */ }
            }
        }
    }

    ExtractedData {
        collections,
        other_types,
    }
}
