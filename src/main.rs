mod handle;
mod service;
mod test;
mod utils;
use handle::*;
use service::*;

use serde::{Deserialize, Serialize};
use std::{fs::File, thread};
use tiny_http::{Header, Response, Server};

const PORT: i32 = 8080;
pub const THUMB_QUALITY: i8 = 10;
const STATIC_DIR: &str = "./web"; // 指定你的静态文件目录


#[derive(Debug, Serialize, Deserialize)]
struct ReqResult<T> {
    success: bool,
    msg: String,
    code: isize,
    data: T,
}

fn main() {
    sqlite::init_tables();

    match Server::http(format!("0.0.0.0:{}", PORT)) {
        Err(_) => println!("start server error;check port is alread used?"),
        Ok(server) => {
            for request in server.incoming_requests() {
                // 使用线程避免慢请求导致服务器阻塞
                thread::spawn(move || {
                    let mut content_type = "";
                    let mut is_api = false;
                    // let mut is_resource = false;
                    let start_url = request.url().trim_start_matches('/');
                    let file_name = match start_url {
                        "" | "index.html" => format!("{}/index.html", STATIC_DIR),
                        // "list.html" => format!("{}list.html", html_dir),
                        somthing => {
                            if somthing.ends_with(".html") {
                                format!("{}/{}", STATIC_DIR, somthing)
                            } else {
                                // content_type = "";
                                is_api = somthing.contains("api/");
                                // is_resource = somthing.contains("res/");
                                if somthing.contains("assets/") && somthing.contains(".js") {
                                    content_type = "text/javascript; charset=UTF-8";
                                }
                                if somthing.contains("assets/") && somthing.contains(".css") {
                                    content_type = "text/css; charset=UTF-8";
                                }
                                if somthing.contains(".svg") {
                                    content_type = "image/svg+xml";
                                }
                                format!("{}/{}", STATIC_DIR, somthing)
                            }
                        }
                    };

                    if is_api {
                        handle_api(request);
                    } else {
                        // println!("file={}", file_name);
                        match File::open(file_name) {
                            Ok(file) => {
                                let mut response = Response::from_file(file);
                                // response.with_status_code(200);
                                response.add_header(
                                    Header::from_bytes(
                                        &b"Access-Control-Allow-Origin"[..],
                                        &b"*"[..],
                                    )
                                    .unwrap(),
                                );
                                if content_type != "" {
                                    response.add_header(
                                        Header::from_bytes(&b"Content-Type"[..], &content_type[..])
                                            .unwrap(),
                                    );
                                }
                                let _ = request.respond(response);
                            }
                            Err(e) => {
                                println!("open file error = {}", e);
                                let _ = request.respond(Response::from_string(
                                    "error".to_string() + &e.to_string(),
                                ));
                            }
                        }
                    }
                });
            }
        }
    }
}