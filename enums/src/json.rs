use std::collections::HashMap;
use std::mem::{size_of, size_of_val};

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

#[test]
fn test_json() {
    let json1 = Json::Boolean(true);
    let json2 = Json::String("ciao".to_string());
    let size1 = size_of_val(&json1);
    let size2 = size_of_val(&json2);
    let size3 = size_of::<Json>();
    assert_eq!(size1, 32);
    assert_eq!(size2, 32);
    assert_eq!(size3, 32);
}