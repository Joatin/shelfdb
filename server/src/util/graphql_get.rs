use crate::util::parse_graphql_response::parse_graphql_response;
use hyper::{Body, Response};
use juniper::http::GraphQLRequest;
use juniper::{DefaultScalarValue, GraphQLType, RootNode};
use std::convert::Infallible;
use std::sync::Arc;

pub async fn graphql_get<Q: GraphQLType<Context = Ctxt>, M: GraphQLType<Context = Ctxt>, Ctxt>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Ctxt,
) -> Result<Response<Body>, Infallible> {
    let query = "".to_owned();
    let operation_name = None;
    let variables = None;

    let graphql_req = GraphQLRequest::<DefaultScalarValue>::new(query, operation_name, variables);

    let result = graphql_req.execute(&root_node, &context);

    Ok(parse_graphql_response(result))
}
