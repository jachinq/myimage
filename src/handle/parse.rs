use std::collections::HashMap;

use urlencoding::decode;

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

// 解析 url 地址，返回 url 和请求参数体的元组
pub fn parse_request(body: &str, is_json: bool) -> HashMap<String, String> {
    // println!("body====={}", body);
    // let parse = decode(&body).expect("UTF-8"); // 解码
    let parse = body;
    // println!("decode====={}", a);
    let mut params = HashMap::new();

    if is_json {
        // extract_known_fields(parse, &params);
        // return params;
    }

    for kv in parse.split("&").collect::<Vec<&str>>().iter() {
        let kv_string = kv.to_string();
        let kv_split = kv_string.split("=").collect::<Vec<&str>>();
        if kv_split.len() == 1 {
            params.insert(kv_split[0].to_string(), "".to_string());
        } else if kv_split.len() > 1 {
            params.insert(
                kv_split[0].to_string(),
                decode(kv_split[1]).expect("UTF-8").to_string(),
            );
        }
    }
    params.remove("");

    // if parse != "" {
    // println!("解析参数 args={:?} params={:?}", parse, params);
    // }

    params
}
