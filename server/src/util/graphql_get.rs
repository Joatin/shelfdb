use crate::util::parse_graphql_response::parse_graphql_response;
use hyper::{Body, Response};
use juniper::http::GraphQLRequest;
use juniper::{DefaultScalarValue, GraphQLTypeAsync, RootNode};
use std::convert::Infallible;
use std::sync::Arc;

pub async fn graphql_get<
    Q: GraphQLTypeAsync<DefaultScalarValue, Context = Ctxt>,
    M: GraphQLTypeAsync<DefaultScalarValue, Context = Ctxt>,
    Ctxt: Send + Sync,
>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Ctxt,
) -> Result<Response<Body>, Infallible>
where
    <Q as juniper::GraphQLType>::TypeInfo: Send + Sync,
    <M as juniper::GraphQLType>::TypeInfo: Send + Sync,
{
    let query = "".to_owned();
    let operation_name = None;
    let variables = None;

    let graphql_req = GraphQLRequest::<DefaultScalarValue>::new(query, operation_name, variables);

    let result = graphql_req.execute_async(&root_node, &context).await;

    Ok(parse_graphql_response(result))
}
