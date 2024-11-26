mod parse;
mod api;
use anyhow::Context;
pub use parse::*;
pub use api::*;
use tiny_http::{Header, Request, Response};

use std::{fs::File, io::Cursor, path::Path};

use crate::STATIC_DIR;

pub fn proc_request(mut request: Request) {
    // http://xx.yy.zz/aaa/bbb/ccc -> aaa/bbb/ccc
    let start_url = request.url().trim_start_matches('/');
    // println!("request url: {}, \nrequest start: {}", request.url(), start_url);
    let is_api = start_url.contains("api/");
    if is_api {
         // 处理api请求
        let _ = match handle_api(&mut request) {
            Ok(response) => request.respond(response),
            Err(e) => {
                println!("handle api error. {}", e);
                request.respond(Response::from_string(e.to_string()))
            },
        };
    } else {
        // 处理静态文件请求
        let _ = match handle_static(&request) {
            Ok(response) => request.respond(response),
            Err(e) => {                
                println!("handle static file error: {}", e);
                request.respond(Response::from_string(e.to_string()))
            }
        };
    }
}

pub fn handle_static(request: &Request) -> anyhow::Result<Response<File>, anyhow::Error> {
    let start_url = request.url().trim_start_matches('/');
    let file_name = match start_url {
        "" => format!("{}/index.html", STATIC_DIR),
        other => format!("{}/{}", STATIC_DIR, other),
    };
    let mut content_type = parse_content_type(start_url);
    let file_name = if Path::new(&file_name).exists() {
        file_name
    } else {
        // 重定向到404页面
        content_type = "text/html; charset=utf-8";
        format!("{}/404.html", STATIC_DIR)
    };
    // println!("get statitc file: {}", file_name);
    let file = File::open(&file_name).with_context(|| format!("open file {} error", file_name))?;
    let mut response = Response::from_file(file);
    response.add_header(build_header("Access-Control-Allow-Origin", "*"));
    response.add_header(build_header("Content-Type", content_type));
    Ok(response)
}

// 转发请求到不同的方法进行处理
pub fn handle_api(request: &mut Request) -> anyhow::Result<Response<Cursor<Vec<u8>>>, anyhow::Error> {
    println!(
        "request: method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    let (url, body) = parse_url(request.url(), &body);

    let result = get_api_data(url, body);

    let mut response = Response::from_string(result);
    response.add_header(build_header("Access-Control-Allow-Origin", "*")); // 允许跨域请求
    response.add_header(build_header("Content-Type", "application/json"));
    // let _ = request.respond(response);
    Ok(response)
}

// 构建header
fn build_header(header: &str, value: &str) -> Header {
    Header::from_bytes(&*header.as_bytes(), &*value.as_bytes()).unwrap()
}
