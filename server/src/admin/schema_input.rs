use uuid::Uuid;

#[derive(GraphQLInputObject)]
pub struct SchemaInput {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>
}