mod handle;
mod service;
mod test;
mod utils;
use handle::*;
use service::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tiny_http::Server;

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

fn main() -> Result<(), anyhow::Error> {
    sqlite::init_tables();

    let server = Server::http(format!("0.0.0.0:{}", PORT));
    if server.is_err() {
        return Err(anyhow::Error::msg(format!(
            "Failed to start server on port {}",
            PORT
        )));
    }
    let server = server.expect("Start server failed.");
    for request in server.incoming_requests() {
        // 使用线程避免慢请求导致服务器阻塞
        std::thread::spawn(move || proc_request(request));
    }
    Ok(())
}