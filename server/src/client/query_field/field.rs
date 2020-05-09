/// This file is where we design the possible queries a user can do
pub enum Field {
    /// The top level node query, all documents in the db can be found through
    /// this query, given the right id
    Node,

    /// The id of the current schema
    SchemaId,

    /// The name of the current schema
    SchemaName,

    /// When this schema was created
    SchemaCreatedAt,

    Document {
        collection_name: String,
    },
    Documents {
        collection_name: String,
    },
    FirstDocumentByField {
        collection_name: String,
        field_name: String,
    },
    FindDocumentsByField {
        collection_name: String,
        field_name: String,
    },
    FirstDocumentByFieldAndField {
        collection_name: String,
        field_name: String,
        second_field_name: String,
    },
    FindDocumentsByFieldAndField {
        collection_name: String,
        field_name: String,
        second_field_name: String,
    },
}
