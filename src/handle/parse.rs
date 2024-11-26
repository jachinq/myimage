use std::{collections::HashMap, str::FromStr};

use urlencoding::decode;

pub struct KvPair {
    k: String,
    v: String,
}
impl FromStr for KvPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, "=");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();
        Ok(KvPair { k: key, v: value })
    }
}

// get 方法获取url
pub fn parse_url<'a>(url: &'a str, body: &'a str) -> (&'a str, &'a str) {
    if url.contains("?") {
        let split = url.split("?");
        let mut url = "";
        let mut body = "";
        let collect: Vec<&str> = split.collect();
        if collect.len() > 1 {
            url = collect[0];
            body = collect[1];
        }
        (url, body)
    } else {
        (url, body)
    }
}

// 解析请求体，返回键值对
pub fn parse_body(body: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for kv in body.split("&").collect::<Vec<&str>>().iter() {
        if let Ok(pair) = kv.parse::<KvPair>() {
            if pair.k.is_empty() {
                continue;
            }
            let v = decode(&pair.v).expect("UTF-8").to_string();
            params.insert(pair.k.to_string(), v);
        }
    }
    params
}

pub fn parse_content_type(start_url: &str) -> &str {
    // content_type.parse().unwrap_or(MIME::Any)
    match start_url {
        _ if start_url.ends_with(".html") => "text/html; charset=UTF-8",
        _ if start_url.ends_with(".css") => "text/css; charset=UTF-8",
        _ if start_url.ends_with(".js") => "text/javascript; charset=UTF-8",
        _ if start_url.ends_with(".json") => "application/json; charset=UTF-8",
        _ if start_url.ends_with(".xml") => "application/xml; charset=UTF-8",
        _ if start_url.ends_with(".pdf") => "application/pdf",
        _ if start_url.ends_with(".png") => "image/png",
        _ if start_url.ends_with(".jpg") => "image/jpeg",
        _ if start_url.ends_with(".jpeg") => "image/jpeg",
        _ if start_url.ends_with(".gif") => "image/gif",
        _ if start_url.ends_with(".svg") => "image/svg+xml; charset=UTF-8",
        _ if start_url.ends_with(".webp") => "image/webp",
        _ if start_url.ends_with(".ogg") => "video/ogg",
        _ if start_url.ends_with(".mp4") => "video/mp4",
        _ => "text/html; charset=UTF-8",
    }
}

