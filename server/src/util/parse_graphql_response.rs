use chrono::{
    DateTime,
    Utc,
};
use hyper::{
    header,
    header::HeaderValue,
    Body,
    Response,
    StatusCode,
};
use juniper::{
    http::GraphQLResponse,
    DefaultScalarValue,
};
use std::time::Instant;

pub fn parse_graphql_response(
    response: GraphQLResponse<DefaultScalarValue>,
    start_time: DateTime<Utc>,
    start_instant: Instant,
) -> Response<Body> {
    if response.is_ok() {
        let json = parse_json(response, start_time, start_instant);
        let mut resp = Response::new(Body::from(json));
        *resp.status_mut() = StatusCode::OK;
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        resp.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );
        resp.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET,POST,OPTIONS"),
        );
        resp
    } else {
        let json = parse_json(response, start_time, start_instant);
        let mut resp = Response::new(Body::from(json));
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        resp.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );
        resp.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET,POST,OPTIONS"),
        );
        resp
    }
}

fn parse_json(
    response: GraphQLResponse<DefaultScalarValue>,
    start_time: DateTime<Utc>,
    start_instant: Instant,
) -> String {
    let end_time = Utc::now();

    let mut value = serde_json::to_value(&response).unwrap();

    let object = value.as_object_mut().unwrap();

    let duration = start_instant.elapsed().as_nanos() as u64;

    object.insert(
        "extensions".to_string(),
        json!({
            "tracing": {
                "version": 1,
                "startTime": start_time,
                "endTime": end_time,
                "duration": duration,
                "parsing": {
                    "startOffset": 0,
                    "duration": 0
                },
                "validation": {
                    "startOffset": 0,
                    "duration": 0
                },
                "execution": {
                    "resolvers": []
                },
            }
        }),
    );

    serde_json::to_string_pretty(&value).expect("Failed to serialize graphql response")
}
