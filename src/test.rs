#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use crate::{parse_body, parse_url};

#[test]
fn test_parse_body() {
    let body = "a=1&b=2&c=3&d=&e=5&=";
    let data = parse_body(body);
    let mut map = HashMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());
    map.insert("c".to_string(), "3".to_string());
    map.insert("d".to_string(), "".to_string());
    map.insert("e".to_string(), "5".to_string());
    assert_eq!(data, map);
}

fn test_parse_url() {
    let url = "http://www.example.com/index.html?a=1&b=2&c=3&d=&e=5&=";
    let (path, query) = parse_url(url, "");
    assert_eq!(path, "/index.html");
    assert_eq!(query, "a=1&b=2&c=3&d=&e=5&=");

}