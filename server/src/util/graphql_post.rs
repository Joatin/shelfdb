use crate::util::parse_graphql_response::parse_graphql_response;
use hyper::body::to_bytes;
use hyper::header::HeaderValue;
use hyper::{header, Body, Request, Response, StatusCode};
use juniper::http::GraphQLRequest;
use juniper::RootNode;
use juniper::{DefaultScalarValue, GraphQLType};
use std::convert::Infallible;
use std::sync::Arc;

pub async fn graphql_post<Q: GraphQLType<Context = Ctxt>, M: GraphQLType<Context = Ctxt>, Ctxt>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Ctxt,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    match to_bytes(req.into_body()).await {
        Ok(body) => match serde_json::from_slice::<GraphQLRequest<DefaultScalarValue>>(&body) {
            Ok(request) => {
                let resp = request.execute(&root_node, &context);

                Ok(parse_graphql_response(resp))
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
