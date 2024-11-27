#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use rusqlite::types::Null;

use crate::{
    parse_body, parse_url, sqlite, utils::{Matcher, Object, Op, VecExt}
};

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

    let body = "";
    let data = parse_body(body);
    assert_eq!(data, HashMap::new());
}

#[test]
fn test_parse_url() {
    let url = "http://www.example.com/index.html?a=1&b=2&c=3&d=&e=5&=";
    let (path, query) = parse_url(url, "");
    assert_eq!(path, "http://www.example.com/index.html");
    assert_eq!(query, "a=1&b=2&c=3&d=&e=5&=");

    let url = "http://www.example.com/api/getAll";
    let (path, query) = parse_url(url, "");
    assert_eq!(path, "http://www.example.com/api/getAll");
    assert_eq!(query, "");
}

#[test]
fn test_matcher() {
    let sql = Matcher::new()
        .and("url", Op::Eq, Object::String("123456".into()))
        .build();
    assert_eq!(sql, "url = '123456'");

    let sql = Matcher::new()
        .and("url", Op::Eq, Object::String("123456".into()))
        .and("time", Op::Eq, Object::Null)
        .build();
    assert_eq!(sql, "time is null");

    let mut list = Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let sql = list.to_sql();
    assert_eq!(sql, "(1,2,3)");

    let mut list = Vec::new();
    list.push(1.1);
    list.push(2.2);
    list.push(3.3);
    let sql = list.to_sql();
    assert_eq!(sql, "(1.1,2.2,3.3)");

    let mut list = Vec::new();
    list.push(true);
    list.push(false);
    list.push(true);
    let sql = list.to_sql();
    assert_eq!(sql, "(true,false,true)");

    let mut list = Vec::new();
    list.push("a");
    list.push("b");
    list.push("c");
    let sql = list.to_sql();
    assert_eq!(sql, "('a','b','c')");

    let sql = Matcher::new()
        .and("url", Op::In, Object::Array(sql))
        .build();
    assert_eq!(sql, "url in ('a','b','c')");
}

#[test]
fn test_sqlite() -> anyhow::Result<()> {
    // let conn = sqlite::connect()?;

    // let mut stmt = conn.prepare("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY)")?;
    // stmt.execute([])?;

    // let mut sql = "INSERT INTO test (id) VALUES ".to_string();

    // for i in 1..10_0000 {
    //     // let mut stmt = conn.prepare("INSERT INTO test (id) VALUES (?)")?;
    //     // stmt.execute([i])?;
    //     sql.push_str(&format!("({}),", i));
    // }
    // sql.pop();

    // let mut stmt = conn.prepare(&sql)?;
    // stmt.execute([])?;

    Ok(())
}