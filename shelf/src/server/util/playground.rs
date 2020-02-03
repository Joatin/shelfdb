use hyper::{Response, Body, header, StatusCode};
use std::convert::Infallible;
use hyper::header::HeaderValue;

pub fn playground(graphql_endpoint: &str) -> Result<Response<Body>, Infallible> {
    let mut resp = Response::new(Body::empty());
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    *resp.body_mut() = Body::from(juniper::http::playground::playground_source(graphql_endpoint));
    Ok(resp)
}