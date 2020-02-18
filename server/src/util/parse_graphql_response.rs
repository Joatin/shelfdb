use hyper::header::HeaderValue;
use hyper::{header, Body, Response, StatusCode};
use juniper::http::GraphQLResponse;
use juniper::DefaultScalarValue;

pub fn parse_graphql_response(response: GraphQLResponse<DefaultScalarValue>) -> Response<Body> {
    if response.is_ok() {
        let json =
            serde_json::to_string_pretty(&response).expect("Failed to serialize graphql response");
        let mut resp = Response::new(Body::from(json));
        *resp.status_mut() = StatusCode::OK;
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        resp
    } else {
        let json =
            serde_json::to_string_pretty(&response).expect("Failed to serialize graphql response");
        let mut resp = Response::new(Body::from(json));
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        resp
    }
}
