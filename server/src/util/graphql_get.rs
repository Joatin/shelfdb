use std::sync::Arc;
use hyper::{Response, Body};
use std::convert::Infallible;
use juniper::http::GraphQLRequest;
use juniper::{DefaultScalarValue, RootNode, GraphQLType};
use crate::util::parse_graphql_response::parse_graphql_response;

pub async fn graphql_get<Q: GraphQLType<Context=Arc<Ctxt>>, M: GraphQLType<Context=Arc<Ctxt>>, Ctxt>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Arc<Ctxt>,
) -> Result<Response<Body>, Infallible> {

    let query = "".to_owned();
    let operation_name = None;
    let variables = None;

    let graphql_req = GraphQLRequest::<DefaultScalarValue>::new(query, operation_name, variables);

    let result = graphql_req.execute(&root_node, &context);

    Ok(parse_graphql_response(result))
}