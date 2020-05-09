use super::QueryField;
use failure::Error;
use inflector::cases::camelcase::to_camel_case;

impl QueryField {
    pub fn from_str(
        field_name: &str,
        collections: &[(String, Vec<String>)],
    ) -> Result<QueryField, Error> {
        match field_name {
            "node" => Ok(QueryField::Node),
            "schemaId" => Ok(QueryField::SchemaId),
            "schemaName" => Ok(QueryField::SchemaName),
            "schemaCreatedAt" => Ok(QueryField::SchemaCreatedAt),
            _ => {
                if let Some((name, _)) = collections
                    .iter()
                    .find(|(name, _fields)| field_name == to_camel_case(name))
                {
                    Ok(QueryField::Document {
                        collection_name: name.to_string(),
                    })
                } else if let Some((name, _)) = collections
                    .iter()
                    .find(|(name, _fields)| field_name == format!("{}s", to_camel_case(name)))
                {
                    Ok(QueryField::Documents {
                        collection_name: name.to_string(),
                    })
                } else {
                    bail!("Unknown field")
                }
            }
        }
    }
}
