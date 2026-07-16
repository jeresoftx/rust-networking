use rust_networking::http::{
    HeaderMap, HttpMethod, HttpParseError, HttpRequest, HttpResponse, HttpVersion, StatusCode,
};

#[test]
fn parses_simple_get_request_line_and_headers() {
    let raw = "GET /academy HTTP/1.1\r\nHost: jeresoft.test\r\nAccept: text/plain\r\n\r\n";
    let request = HttpRequest::parse(raw).unwrap();

    assert_eq!(request.method(), HttpMethod::Get);
    assert_eq!(request.path(), "/academy");
    assert_eq!(request.version(), HttpVersion::Http11);
    assert_eq!(request.headers().get("host"), Some("jeresoft.test"));
    assert_eq!(request.headers().get("ACCEPT"), Some("text/plain"));
    assert!(request.body().is_empty());
}

#[test]
fn invalid_method_is_rejected() {
    let raw = "FETCH /academy HTTP/1.1\r\nHost: jeresoft.test\r\n\r\n";
    let error = HttpRequest::parse(raw).unwrap_err();

    assert_eq!(error, HttpParseError::InvalidMethod("FETCH".to_string()));
}

#[test]
fn response_contains_status_code_headers_and_body() {
    let response = HttpResponse::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_body("hola".as_bytes().to_vec());

    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(response.status().code(), 200);
    assert_eq!(response.reason_phrase(), "OK");
    assert_eq!(response.headers().get("content-type"), Some("text/plain"));
    assert_eq!(response.body(), b"hola");
}

#[test]
fn cache_helpers_store_cache_control_and_etag() {
    let headers = HeaderMap::new()
        .with_cache_control("max-age=60")
        .with_etag("\"lesson-1\"");

    assert_eq!(headers.cache_control(), Some("max-age=60"));
    assert_eq!(headers.etag(), Some("\"lesson-1\""));
}
