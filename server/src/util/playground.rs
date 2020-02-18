use hyper::header::HeaderValue;
use hyper::{header, Body, Response, StatusCode};
use std::convert::Infallible;

/// Takes care of rendering the playground graphql explorer
///
/// # Arguments
///
/// * `graphql_endpoint` - The absolute path the graphql endpoint is located at
pub fn playground(graphql_endpoint: &str) -> Result<Response<Body>, Infallible> {
    let mut resp = Response::new(Body::empty());
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    *resp.body_mut() = Body::from(juniper::http::playground::playground_source(
        graphql_endpoint,
    ));
    Ok(resp)
}
