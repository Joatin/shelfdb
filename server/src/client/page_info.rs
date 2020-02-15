

#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: String,
    pub end_cursor: String
}