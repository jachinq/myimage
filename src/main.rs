use image::GenericImageView;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{remove_file, File},
    path::Path,
    thread,
};
use tiny_http::{Header, Request, Response, Server};
use urlencoding::decode;

const PORT: i32 = 8080;
const THUMB_QUALITY: i8 = 10;
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
    Data::add(data).json()
}

fn delete(params: HashMap<String, String>) -> String {
    let url = utils::get_value(&params, "url", String::new());
    Data::delete_single(&url).json()
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
        if !result.is_success() {
            success = false;
            results_err.push(result);
            continue;
        }
        results_ok.push(result);
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
    return Data::get_all(0, 0, start, limit).json();
}

#[derive(Debug, Serialize, Deserialize)]
struct ReqResult<T> {
    success: bool,
    msg: String,
    code: isize,
    data: T,
}

impl<T: Serialize> ReqResult<T> {
    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn json(&mut self) -> String {
        if let Ok(json) = serde_json::to_string(self) {
            json
        } else {
            "".to_string()
        }
    }

    pub fn conn_error(data: T) -> Self {
        ReqResult {
            success: false,
            code: -120,
            msg: "数据库连接失败".to_string(),
            data,
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

#[derive(Debug, Serialize, Deserialize)]
struct UploadArg {
    name: String,
    r#type: String,
    size: String,
    modify: String,
    src: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageData {
    list: Vec<Data>,
    total: i64,
}
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    url: String,
    thumb: String,
    time: i64,
    size: i64,
    width: i64,
    height: i64,
}

impl Data {
    pub fn add(data: Self) -> ReqResult<Self> {
        // 业务逻辑
        let add = |conn: Connection, data: Data| {
            let fields = "name,url,thumb,time,size,width,height";
            let values = sqlite::turn_values(fields.to_string());

            let (success, msg, code, rows) = sqlite::fmt_result(conn.execute(
                &format!("INSERT INTO picture ({}) VALUES ({})", fields, values),
                params![
                    data.name,
                    data.url,
                    data.thumb,
                    data.time,
                    data.size,
                    data.width,
                    data.height
                ],
            ));
            let _ = conn.close();

            println!(
                "add picture end; data={:?} rows={} success={} msg={}",
                data, rows, success, msg
            );
            ReqResult {
                success,
                code,
                msg: "上传成功".to_string(),
                data,
            }
        };

        // 成功连接数据库后执行业务逻辑
        match sqlite::connect() {
            Ok(conn) => add(conn, data),
            Err(_) => ReqResult::conn_error(data),
        }
    }

    pub fn get_by_url(url: &str, is_thumb: bool) -> Option<Self> {
        let mut sql = "SELECT * FROM picture where url = ?1".to_string();
        if is_thumb {
            sql = "SELECT * FROM picture where thumb = ?1".to_string();
        }

        let conn = sqlite::connect();
        if conn.is_err() {
            return None;
        }

        let conn = conn.unwrap();

        let mut stmt = conn.prepare(&sql).unwrap();
        let iter = stmt.query_map(params![url], |row| {
            let data = Self {
                name: row.get(0)?,
                url: row.get(1)?,
                thumb: row.get(2)?,
                time: row.get(3)?,
                size: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
            };
            // println!("data={}", data.json());
            Ok(data)
        });
        if iter.is_err() {
            return None;
        }
        let iter = iter.unwrap();

        let mut single = None;
        for some in iter {
            if some.is_ok() {
                single = Some(some.unwrap());
                break;
            }
        }
        single
    }

    pub fn get_all(beg: isize, end: isize, start: isize, limit: isize) -> ReqResult<PageData> {
        let mut condition = String::new();
        if beg > 0 {
            if condition != "" {
                condition.push_str(" and ");
            }
            condition.push_str("`create`>=");
            condition.push_str(&beg.to_string());
        }
        if end > 0 {
            if condition != "" {
                condition.push_str(" and ");
            }
            condition.push_str("`create`<=");
            condition.push_str(&end.to_string());
        }

        let mut sql = "SELECT * FROM picture".to_string();
        if condition != "" {
            sql += " where ";
            sql += &condition;
        }
        sql += " order by time desc ";
        if limit > 0 {
            sql += " limit ";
            sql += &start.to_string();
            sql += ",";
            sql += &limit.to_string();
            sql += ";";
        }

        let mut count_sql = "SELECT COUNT(*) as count FROM picture".to_string();
        if condition != "" {
            count_sql += " where ";
            count_sql += &condition;
        }

        println!("sql==={sql} count_sql={count_sql}");

        let get_all = |conn: Connection| {
            let mut stmt = conn.prepare(&sql).unwrap();

            let iter = stmt
                .query_map(params![], |row| {
                    let data = Self {
                        name: row.get(0)?,
                        url: row.get(1)?,
                        thumb: row.get(2)?,
                        time: row.get(3)?,
                        size: row.get(4)?,
                        width: row.get(5)?,
                        height: row.get(6)?,
                    };
                    // println!("data={}", data.json());
                    Ok(data)
                })
                .unwrap();

            let mut total = 0;
            let _ = conn.query_row(&count_sql, [], |row| {
                println!("row={:?}", row);
                total = row.get_ref(0)?.as_i64()?;
                Ok(())
            });

            let mut list: Vec<Self> = Vec::new();

            for some in iter {
                match some {
                    Ok(data) => list.push(data),
                    Err(e) => {
                        println!("e={}", e);
                    }
                }
            }

            ReqResult {
                success: true,
                code: 0,
                msg: "查询成功".to_string(),
                data: PageData { list, total },
            }
        };

        match sqlite::connect() {
            Ok(conn) => get_all(conn),
            Err(_) => ReqResult::conn_error(PageData {
                list: vec![],
                total: 0,
            }),
        }
    }

    pub fn del(url: String, is_thumb: bool) -> ReqResult<String> {
        let del = |conn: Connection, thumb_url: String| {
            let (success, msg, code, rows) = if is_thumb {
                sqlite::fmt_result(
                    conn.execute("DELETE FROM picture WHERE thumb = ?1", params![thumb_url]),
                )
            } else {
                sqlite::fmt_result(
                    conn.execute("DELETE FROM picture WHERE url = ?1", params![thumb_url]),
                )
            };
            println!(
                "del picture end; rows={:?} thumb={:?} success={} msg={}",
                rows, thumb_url, rows, msg
            );
            let msg = if success {
                "删除成功, 行数=".to_string() + &rows.to_string()
            } else {
                msg
            };
            ReqResult {
                success,
                code,
                msg,
                data: thumb_url,
            }
        };
        match sqlite::connect() {
            Ok(conn) => del(conn, url),
            Err(_) => ReqResult::conn_error(url),
        }
    }

    pub fn delete_single(url_arg: &str) -> ReqResult<String> {
        if url_arg.is_empty() {
            return ReqResult::error("url不能为空", String::new());
        }
        let is_thumn = url_arg.contains("thumb");
        let data = Data::get_by_url(url_arg, is_thumn);
        if data.is_none() {
            return ReqResult::error("找不到需要删除的图片", String::new());
        }

        let data = data.unwrap();
        let url = data.url.replace("./res", "./web/res");
        let thumb = data.thumb.replace("./res", "./web/res");

        let ok = remove_file(Path::new(&thumb)); // 先删掉缩略图
        if ok.is_err() {
            println!("delete thumb err={}", ok.unwrap_err());
            return ReqResult::error("删除缩略图失败", thumb);
        }

        let ok = remove_file(Path::new(&url));
        if ok.is_err() {
            println!("delete img err={}", ok.unwrap_err());
            return ReqResult::error("删除图片失败",  url);
        }
        Data::del(url_arg.to_string(), is_thumn)
    }
}

mod sqlite {
    use rusqlite::{Connection, Error, Result};

    use crate::utils;

    pub fn connect() -> Result<Connection> {
        utils::check_dir_and_create("./data");
        Connection::open("./data/data.db")
    }

    pub fn init_tables() {
        let init_tables = |conn: Connection| {
            let result: Result<(), Error> = conn.execute_batch(
                "BEGIN;
                CREATE TABLE if not exists picture (
                    name TEXT NOT NULL DEFAULT '',
                    url TEXT NOT NULL DEFAULT '',
                    thumb TEXT NOT NULL DEFAULT '',
                    time INTEGER NOT NULL DEFAULT 0,
                    size INTEGER NOT NULL DEFAULT 0,
                    width INTEGER NOT NULL DEFAULT 0,
                    height INTEGER NOT NULL DEFAULT 0
                );
            COMMIT;",
            );
            match result {
                Err(e) => println!("init table error={e}"),
                _ => {}
            };
        };

        match connect() {
            Ok(conn) => init_tables(conn),
            Err(e) => println!("db connect error, {e}"),
        }
    }

    // 匹配字段和占位符
    pub fn turn_values(fields: String) -> String {
        let size = fields.split(",").collect::<Vec<&str>>().len() + 1;
        let mut values = String::new();
        for num in 1..size {
            values.push_str(&format!("?{}", num));
            if num == size - 1 {
                break;
            }
            values.push_str(",");
        }
        values
    }

    pub fn fmt_result(result: Result<usize, Error>) -> (bool, String, isize, usize) {
        let mut tunple = (true, "成功".to_string(), 0, 0);
        match result {
            Ok(size) => tunple.3 = size,
            Err(e) => {
                tunple.0 = false;
                tunple.1 = e.to_string();
            }
        }
        if !tunple.0 {
            tunple.2 = -1;
        }
        tunple
    }
}

#[allow(dead_code)]
mod utils {
    use super::{STATIC_DIR, THUMB_QUALITY};
    use image::{DynamicImage, GenericImageView};
    use std::collections::HashMap;
    use std::io::Write;
    use std::str::FromStr;
    use std::time::Instant;
    use std::{fs::File, io::BufWriter, path::Path};

    /// 检查路径是否存在，不存在则创建路径
    pub fn check_dir_and_create(path: &str) {
        if Path::new(path).exists() {
            return;
        }
        match std::fs::create_dir_all(path) {
            Err(err) => println!("create path {} error: {}", path, err),
            Ok(_) => println!("create path {} ok", path),
        }
    }

    // 获取当前程序运行路径
    pub fn current_dir() -> String {
        match std::env::current_dir() {
            Ok(path) => path.display().to_string().replace("\\", "/"),
            Err(_) => ".".to_string(),
        }
    }

    /// 获取存放路径和 url
    /// 普通文件：STATIC_DIR/res[/appid]/year_month
    /// 缩略图：STATIC_DIR/res[/appid]/thumb/year_month
    /// 普通文件url：./res[/appid]/year_month/uuid8bit.ext
    /// 缩略图url：./res[/appid]/thumb/year_month/uuid8bit.ext
    pub fn get_file_path_and_url(
        is_thumb: bool,
        appid: Option<&String>,
        uuid: &str,
        ext: &str,
    ) -> (String, String) {
        let mut path = String::new();
        path.push_str("/res");
        if let Some(appid) = appid {
            path.push_str("/");
            path.push_str(appid);
        }
        if is_thumb {
            path.push_str("/thumb");
        }
        let fmt = "%Y%m";
        let year_month = chrono::Local::now().format(fmt).to_string();
        path.push_str("/");
        path.push_str(&year_month);

        // 此时 path 应该是 /res[/appid][/thumb]/year_month
        // 对应的就是文件夹，检查一下是否已存在并自动创建
        check_dir_and_create(&format!("{}{}", STATIC_DIR, path));

        path.push_str("/");
        path.push_str(uuid);
        // if is_thumb {
        //     path.push_str("_thumb");
        // }
        path.push_str(".");
        path.push_str(ext);
        let url = format!(".{}", path);
        let file_path = format!("{}{}", STATIC_DIR, path);
        (file_path, url)
    }

    // 从 map 中获取指定 key 的值，并转换为指定类型，如果转换失败则返回默认值
    pub fn get_value<T: ToString + FromStr>(
        params: &HashMap<String, String>,
        key: &str,
        default_value: T,
    ) -> T {
        let binding = default_value.to_string();
        let size = params.get(key).or(Some(&binding)).unwrap();
        if let Ok(size) = size.parse() {
            size
        } else {
            default_value
        }
    }

    // 缩小图片
    pub fn resize_image(img: Box<DynamicImage>, process_origin: bool) -> Box<DynamicImage> {
        let thumbnail_size = 300;
        let (width, height) = img.dimensions();

        if width <= thumbnail_size || height <= thumbnail_size {
            return img;
        }

        let mut nwidth = width;
        let mut nheight = height;

        if width > height {
            // h=316 target_h=250 w=1415
            // target_w=250/316*1415
            nwidth = (thumbnail_size as f32 / height as f32 * width as f32) as u32;
        }
        if height > width {
            nheight = (thumbnail_size as f32 / width as f32 * height as f32) as u32;
        }

        // 存储原图的话，先压缩原图，再生成缩略图，基本能保证最终的缩略图大小范围在 20k 内
        if process_origin {
            if let Ok(img) = compress_img(&img, THUMB_QUALITY) {
                if let Ok(img) = image::load_from_memory(&img) {
                    return Box::new(img.resize(
                        nwidth,
                        nheight,
                        image::imageops::FilterType::Nearest,
                    ));
                }
            }
        }

        // 将原始尺寸的图片缩小到指定尺寸
        return Box::new(img.resize(nwidth, nheight, image::imageops::FilterType::Nearest));
    }

    // 根据 img 压缩成 webp 格式
    pub fn compress_img(img: &DynamicImage, qulity: i8) -> Result<Vec<u8>, String> {
        match webp::Encoder::from_image(img) {
            Err(err) => Err(err.to_string()),
            Ok(encoder) => {
                let webp = encoder.encode(qulity as f32);
                Ok(webp.to_vec())
            }
        }
    }

    // 保存图片到指定路径
    pub fn save_img(buf: &[u8], file_path: &str) -> Result<usize, String> {
        let start_time = std::time::Instant::now();
        let create_result = File::create(file_path);
        if let Err(err) = create_result {
            return Err(err.to_string());
        }
        let file = create_result.expect("create file error.");
        let write_result = BufWriter::new(file).write(&buf);
        if let Err(err) = write_result {
            return Err(err.to_string());
        }
        let size = write_result.expect("write img error.");
        let info = format!("save img path={}, size={}", file_path, size);
        log_time_used(start_time, &info);
        Ok(size)
    }

    pub fn log_time_used(start_time: Instant, info: &str) {
        let end_time = Instant::now();
        println!(
            "Info: {} time={:?}",
            info,
            end_time.duration_since(start_time)
        );
    }
}
