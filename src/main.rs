mod service;
mod utils;
use service::*;

use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::File, thread};
use tiny_http::{Header, Request, Response, Server};
use urlencoding::decode;

const PORT: i32 = 8080;
pub const THUMB_QUALITY: i8 = 10;
const STATIC_DIR: &str = "./web"; // 指定你的静态文件目录

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

fn handle_api(mut request: Request) {
    println!(
        "request: method: {:?}, url: {:?}", // headers: {:?}",
        request.method(),
        request.url(),
        // request.headers()
    );

    let headers = request.headers();
    let mut content_type = "";
    for ele in headers.iter() {
        // println!("{:?}", ele);
        if ele.field.equiv("Content-Type") {
            content_type = ele.value.as_str();
        }
    }
    // println!("Content-Type: {}", content_type);
    let is_json = content_type.contains("json");

    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    let parse_url = parse_url(request.url(), &body);
    let url = parse_url.0;
    let body = parse_url.1;

    let params = parse_request(&body, is_json);
    let result: String = handle_request(url, params);
    let mut response: Response<std::io::Cursor<Vec<u8>>> = Response::from_string(result);
    response
        .add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
    response
        .add_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
    let _ = request.respond(response);
}

// get 方法获取url
fn parse_url<'a>(url: &'a str, body: &'a str) -> (&'a str, &'a str) {
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
fn parse_request(body: &str, is_json: bool) -> HashMap<String, String> {
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

fn _extract_known_fields(
    json_string: &str,
    _map: &HashMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_json: Value = serde_json::from_str(json_string)?;

    // 假设我们知道有一些特定的键要查找
    // let known_keys = ["name", "age", "address"];

    // 检查是否是一个JSON对象
    if let Some(json_obj) = parsed_json.as_object() {
        for known_key in json_obj.keys() {
            // 尝试从JSON对象中获取已知键的值
            if let Some(value) = json_obj.get(known_key) {
                println!("{}: {:?}", known_key, value);

                // 这里可以进一步处理值，例如将其转换为特定的类型
                match value {
                    Value::String(s) => println!("{} is a string: {}", known_key, s),
                    Value::Number(n) => println!("{} is a number: {}", known_key, n),
                    Value::Bool(b) => println!("{} is a boolean: {}", known_key, b),
                    // 其他可能的值类型...
                    _ => println!("{} has a different type", known_key),
                }
            }
        }
    }

    Ok(())
}

// 转发请求到不同的方法进行处理
fn handle_request(url: &str, params: HashMap<String, String>) -> String {
    match &url.replace("api/", "") as &str {
        "/test" => test(),
        "/getAll" => get_all(params),
        "/upload" => upload(params),
        "/delete" => delete(params),
        "/deleteAll" => delete_all(params),
        _ => method_not_found(params, url),
    }
}

fn method_not_found(params: HashMap<String, String>, url: &str) -> String {
    println!("not match;params={params:?} url={url}");
    ReqResult {
        success: false,
        msg: "找不到方法".to_string(),
        code: -2,
        data: "".to_string(),
    }
    .json()
}

fn test() -> String {
    println!("匹配到了 /hey");

    ReqResult {
        success: true,
        msg: "请求成功".to_string(),
        code: 0,
        data: "hello tiny http".to_string(),
    }
    .json()
}

// 上传图片
fn upload(params: HashMap<String, String>) -> String {
    // println!("{:?}", params);
    let quality = utils::get_value(&params, "quality", 40);
    // 图片存放目录，默认为 ./web/res/
    let appid = params.get("appid");
    // 先判断是否需要压缩图片
    let origin_upload = quality >= 100 || quality < 10;

    let file_type = if origin_upload {
        utils::get_value(&params, "type", "png".to_string()).replace("image/", "")
        // match params.get("type") {
        //     Some(file_type) => file_type.replace("image/", ""),
        //     None => "png".to_string(),
        // }
    } else {
        "webp".to_string()
    };
    println!(
        "匹配到了 /upload, qulity={} file_type={} use_origin={}",
        quality, file_type, origin_upload
    );

    let name = match params.get("name") {
        Some(name) => name.to_string(),
        None => "".to_string(),
    };

    let size = utils::get_value(&params, "size", 0);

    let data = params.get("data");
    if data.is_none() {
        return ReqResult::error("data 参数为空", data).json();
    }

    let data = params.get("data").expect("query data is none.");
    let start_time = std::time::Instant::now();
    let mut buf = image_base64::from_base64(data.to_string());
    utils::log_time_used(start_time, "change base64");

    let uuid = &uuid::Uuid::new_v4().to_string()[0..8]; // 截取前8位作为文件名
    let (file_path, url) = utils::get_file_path_and_url(false, appid, uuid, &file_type);
    let (file_path_thumb, mut url_thumb) =
        utils::get_file_path_and_url(true, appid, uuid, &file_type);
    // println!("DEBUG: file_path={}", file_path);
    // println!("DEBUG: file_path_thumb={}", file_path_thumb);
    // println!("DEBUG: url={}", url);
    // println!("DEBUG: url_thumb={}", url_thumb);

    // 从参数中加载图片
    let start_time = std::time::Instant::now();
    let load_img_result = image::load_from_memory(&buf);
    utils::log_time_used(start_time, "load img from query");
    if let Err(e) = load_img_result {
        println!("load img from query err {}", e);
        let msg = match e {
            image::ImageError::Unsupported(_) => &format!(
                "不支持的图片类型: {}",
                utils::get_value(&params, "type", "png".to_string()).replace("image/", "")
            ),
            image::ImageError::Decoding(_) => "解码错误，请检查图片格式是否正确",
            image::ImageError::Encoding(_) => "编码错误，请检查图片格式是否正确",
            image::ImageError::Parameter(_) => "参数错误，例如图像的维度不正确",
            image::ImageError::Limits(_) => "图像大小超过限制",
            image::ImageError::IoError(_) => "IO错误，请检查文件是否存在或权限是否正确",
        };
        return ReqResult::error(msg, data).json();
    }
    let img = load_img_result.expect("load img from query error.");

    // 压缩原图
    if !origin_upload {
        let start_time = std::time::Instant::now();
        let compress_result = utils::compress_img(&img, quality);
        if let Err(e) = compress_result {
            println!("compress img err={}", e);
            return ReqResult::error("压缩图片失败", data).json();
        }
        let webp = compress_result.expect("compress img error.");
        utils::log_time_used(start_time, "compress");

        buf = webp
    }

    let upload_result = match utils::save_img(&buf, &file_path) {
        Err(e) => Err(e.to_string()),
        Ok(save_size) => Ok(save_size),
    };

    if let Err(err) = upload_result {
        println!("save err {}", err);
        return ReqResult::error("保存图片失败", data).json();
    }

    let real_size = upload_result.expect("upload error.");
    if origin_upload && real_size != size {
        println!(
            "Warn: size not match, save_size={}, query_size={}",
            real_size, size
        );
    }

    let (width, height) = img.dimensions();
    let mut proc_thumb_faile = false;
    let img_box = Box::new(img);
    let img = utils::resize_image(img_box, origin_upload);
    match utils::compress_img(&img, THUMB_QUALITY) {
        Err(result) => {
            println!("compress img err={}", result);
            proc_thumb_faile = true;
        }
        Ok(data) => {
            if let Err(e) = utils::save_img(&data, &file_path_thumb) {
                println!("save thumbnail err={}", e);
                proc_thumb_faile = true;
            }
        }
    }
    if proc_thumb_faile {
        url_thumb = url.clone();
    }

    let data = Data {
        name,
        url,
        thumb: url_thumb,
        time: chrono::Local::now().timestamp(),
        size: real_size as i64,
        width: width as i64,
        height: height as i64,
    };
    // 上传成功，索引数据落盘
    // match Data::add(data) {
    //     Ok(data) => ReqResult::success("上传成功", data).json(),
    //     Err(e) => ReqResult::error("上传失败", e).json(),
    // }
    proc_result(Data::add(data)).json()
}

fn delete(params: HashMap<String, String>) -> String {
    let url = utils::get_value(&params, "url", String::new());
    proc_result(Data::delete_single(&url)).json()
}

fn delete_all(params: HashMap<String, String>) -> String {
    let urls = utils::get_value(&params, "url", String::new());
    if urls.trim().is_empty() {
        return ReqResult::error("请指定正确的url", urls).json();
    }

    let urls = urls.replace("[", "");
    let urls = urls.replace("]", "");
    let urls = urls.replace("\"", "");
    let url_split = urls.split(",");

    let mut success = true;
    let mut results_ok = vec![];
    let mut results_err = vec![];
    for url in url_split {
        let result = Data::delete_single(url);
        if !result.is_ok() {
            success = false;
            results_err.push(url);
            continue;
        }
        results_ok.push(url);
    }

    if success {
        ReqResult::success("删除成功", results_ok).json()
    } else {
        ReqResult::error("删除失败", results_err).json()
    }
}

fn get_all(params: HashMap<String, String>) -> String {
    let current = match params.get("current") {
        Some(current) => current.parse().unwrap(),
        None => 1,
    };
    let limit = match params.get("limit") {
        Some(current) => current.parse().unwrap(),
        None => 0,
    };
    let start = (current - 1) * limit;
    return proc_result(Data::get_all(0, 0, start, limit)).json();
}

#[derive(Debug, Serialize, Deserialize)]
struct ReqResult<T> {
    success: bool,
    msg: String,
    code: isize,
    data: T,
}

impl<T: Serialize> ReqResult<T> {

    pub fn json(&mut self) -> String {
        if let Ok(json) = serde_json::to_string(self) {
            json
        } else {
            "".to_string()
        }
    }

    pub fn success(msg: &str, data: T) -> Self {
        ReqResult {
            success: true,
            code: 0,
            msg: msg.to_string(),
            data,
        }
    }

    pub fn error(msg: &str, data: T) -> Self {
        ReqResult {
            success: false,
            code: -1,
            msg: msg.to_string(),
            data,
        }
    }
}
