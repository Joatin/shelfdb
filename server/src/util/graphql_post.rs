use std::sync::Arc;
use hyper::{Request, Body, Response, header, StatusCode};
use std::convert::Infallible;
use juniper::http::GraphQLRequest;
use juniper::{DefaultScalarValue, GraphQLType};
use juniper::RootNode;
use crate::util::parse_graphql_response::parse_graphql_response;
use hyper::header::HeaderValue;
use hyper::body::to_bytes;

pub async fn graphql_post<Q: GraphQLType<Context=Arc<Ctxt>>, M: GraphQLType<Context=Arc<Ctxt>>, Ctxt>(
    root_node: Arc<RootNode<'_, Q, M>>,
    context: Arc<Ctxt>,
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    match to_bytes(req.into_body()).await {
        Ok(body) => {

            match serde_json::from_slice::<GraphQLRequest<DefaultScalarValue>>(&body) {
                Ok(request) => {
                    let resp = request.execute(&root_node, &context);

                    Ok(parse_graphql_response(resp))
                },
                Err(_e) => {
                    let mut resp = Response::new(Body::from(serde_json::to_string_pretty("Body was not valid json").unwrap()));
                    *resp.status_mut() = StatusCode::BAD_REQUEST;
                    resp.headers_mut().insert(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json")
                    );
                    Ok(resp)
                }
            }
        },
        Err(_e) => {
            let mut resp = Response::new(Body::from(serde_json::to_string_pretty("Failed to parse body").unwrap()));
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json")
            );
            Ok(resp)
        }
    }

}