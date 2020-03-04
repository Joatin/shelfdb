use crate::util::parse_graphql_response::parse_graphql_response;
use chrono::Utc;
use hyper::{
    body::to_bytes,
    header,
    header::HeaderValue,
    Body,
    Request,
    Response,
    StatusCode,
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

pub async fn graphql_post<
    Q: GraphQLTypeAsync<DefaultScalarValue, Context = Ctxt>,
    M: GraphQLTypeAsync<DefaultScalarValue, Context = Ctxt>,
    Ctxt: Send + Sync,
>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Ctxt,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible>
where
    <Q as juniper::GraphQLType>::TypeInfo: Send + Sync,
    <M as juniper::GraphQLType>::TypeInfo: Send + Sync,
{
    let start_time = Utc::now();
    let start_instant = Instant::now();
    match to_bytes(req.into_body()).await {
        Ok(body) => match serde_json::from_slice::<GraphQLRequest<DefaultScalarValue>>(&body) {
            Ok(request) => {
                let resp = request.execute_async(&root_node, &context).await;

                Ok(parse_graphql_response(resp, start_time, start_instant))
            }
            Err(_e) => {
                let mut resp = Response::new(Body::from(
                    serde_json::to_string_pretty("Body was not valid json").unwrap(),
                ));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                resp.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                Ok(resp)
            }
        },
        Err(_e) => {
            let mut resp = Response::new(Body::from(
                serde_json::to_string_pretty("Failed to parse body").unwrap(),
            ));
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            Ok(resp)
        }
    }
}
