use crate::util::parse_graphql_response::parse_graphql_response;
use chrono::Utc;
use hyper::{
    Body,
    Response,
};
use juniper::{
    http::GraphQLRequest,
    DefaultScalarValue,
    GraphQLTypeAsync,
    RootNode,
};
use std::{
    convert::Infallible,
    sync::Arc,
    time::Instant,
};

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
    let start_time = Utc::now();
    let start_instant = Instant::now();
    let query = "".to_owned();
    let operation_name = None;
    let variables = None;

    let graphql_req = GraphQLRequest::<DefaultScalarValue>::new(query, operation_name, variables);

    let result = graphql_req.execute_async(&root_node, &context).await;

    Ok(parse_graphql_response(result, start_time, start_instant))
}
