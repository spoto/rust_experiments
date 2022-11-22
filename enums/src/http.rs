#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[test]
fn test_http_status() {
    assert_eq!(200, HttpStatus::Ok as u32);
    assert_eq!(304, HttpStatus::NotModified as u32);
    assert_eq!(404, HttpStatus::NotFound as u32);
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[test]
fn test_http_status_from_u32() {
    assert_eq!(http_status_from_u32(304), Some(HttpStatus::NotModified));
    assert_eq!(http_status_from_u32(104), None);
}