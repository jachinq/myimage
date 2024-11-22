use image::{DynamicImage, GenericImageView};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{remove_file, File},
    io::{BufWriter, Write},
    path::Path,
    thread,
};
use tiny_http::{Header, Request, Response, Server};
use urlencoding::decode;

fn main() {
    sqlite::init_tables();

    match Server::http("0.0.0.0:10016") {
        Err(_) => println!("start server error;check port is alread used?"),
        Ok(server) => {
            let html_dir = "./web/"; // 指定你的静态文件目录
            for request in server.incoming_requests() {
                // 使用线程避免慢请求导致服务器阻塞
                thread::spawn(move || {
                    let mut content_type = "";
                    let mut is_api = false;
                    // let mut is_resource = false;
                    let start_url = request.url().trim_start_matches('/');
                    let file_name = match start_url {
                        "" | "index.html" => format!("{}index.html", html_dir),
                        // "list.html" => format!("{}list.html", html_dir),
                        somthing => {
                            if somthing.ends_with(".html") {
                                format!("{}{}", html_dir, somthing)
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
                                // if !is_api {
                                // println!("other file={}", somthing);
                                // }
                                format!("{}{}", html_dir, somthing)
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
        "/compressUpload" => compress_upload(params),
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

fn upload(params: HashMap<String, String>) -> String {
    println!("匹配到了 /upload");
    // println!("{:?}", params);

    let file_type = match params.get("type") {
        Some(file_type) => file_type.replace("image/", ""),
        None => "png".to_string(),
    };
    let name = match params.get("name") {
        Some(name) => name.to_string(),
        None => "".to_string(),
    };

    let default_value = "0".to_string();
    let size = params.get("size").or(Some(&default_value)).unwrap();
    let size = if let Ok(size) = size.parse() { size } else { 0 };

    if let Some(data) = params.get("data") {
        println!("data={}", file_type);
        let buf = image_base64::from_base64(data.to_string());

        let fmt = "%Y%m";
        let year_month = chrono::Local::now().format(fmt).to_string();
        let path = format!("{}/web/res/{}", utils::current_dir(), year_month);
        utils::check_dir_and_create(&path);

        let uuid = &uuid::Uuid::new_v4().to_string()[0..8];
        let url = format!("./res/{}/{}.{}", year_month, uuid, file_type);
        let file_path = format!("./web/res/{}/{}.{}", year_month, uuid, file_type);
        let mut url_thumb = format!("./res/{}/{}_thumb.{}", year_month, uuid, file_type);
        let file_path_thumbnail = format!("./web/res/{}/{}_thumb.{}", year_month, uuid, file_type);
        // let file_path = format!("{}/web/res/{}.{}", current_dir(), uuid, file_type);

        if let Ok(file) = File::create(file_path.clone()) {
            match BufWriter::new(file).write_all(&buf) {
                Ok(_) => {
                    let (mut width, mut height) = (0, 0);

                    let mut proc_thumb_faile = false;
                    if let Ok(img) = image::load_from_memory(&buf) {
                        width = img.dimensions().0;
                        height = img.dimensions().1;
                        if let Some(img) = compress_image(&img, size) {
                            if let Err(result) = save_thumbnail(&img, &file_path_thumbnail) {
                                println!("save thumbnail err={}", result);
                                proc_thumb_faile = true;
                            }
                        } else {
                            proc_thumb_faile = true;
                        }
                    } else {
                        proc_thumb_faile = true;
                    }
                    if proc_thumb_faile {
                        url_thumb = url.clone();
                    }

                    let data = Data {
                        name,
                        url,
                        thumb: url_thumb,
                        time: chrono::Local::now().timestamp(),
                        size: size as i64,
                        width: width as i64,
                        height: height as i64,
                    };
                    return Data::add(data).json();
                }
                Err(e) => println!("save err {}", e),
            }
        }
    }

    ReqResult {
        success: false,
        msg: "上传失败".to_string(),
        code: -1,
        data: "".to_string(),
    }
    .json()
}

// 压缩上传
fn compress_upload(params: HashMap<String, String>) -> String {
    println!("匹配到了 /upload");
    // println!("{:?}", params);

    let file_type = "webp";
    let name = match params.get("name") {
        Some(name) => name.to_string(),
        None => "".to_string(),
    };

    // let default_value = "0".to_string();
    // let size = params.get("size").or(Some(&default_value)).unwrap();
    // let size = if let Ok(size) = size.parse() { size } else { 0 };

    if let Some(data) = params.get("data") {
        println!("data={}", file_type);
        let buf = image_base64::from_base64(data.to_string());

        let fmt = "%Y%m";
        let year_month = chrono::Local::now().format(fmt).to_string();
        let path = format!("{}/web/res/{}", utils::current_dir(), year_month);
        utils::check_dir_and_create(&path);

        let uuid = &uuid::Uuid::new_v4().to_string()[0..8];
        let url = format!("./res/{}/{}.{}", year_month, uuid, file_type);
        let file_path = format!("./web/res/{}/{}.{}", year_month, uuid, file_type);
        let mut url_thumb = format!("./res/{}/{}_thumb.{}", year_month, uuid, file_type);
        let file_path_thumbnail = format!("./web/res/{}/{}_thumb.{}", year_month, uuid, file_type);
        // let file_path = format!("{}/web/res/{}.{}", current_dir(), uuid, file_type);

        let start_load_from_query = std::time::Instant::now();
        if let Ok(file) = File::create(file_path.clone()) {
            match image::load_from_memory(&buf) {
                Ok(img) => {
                    let end_load_from_query = std::time::Instant::now();
                    println!(
                        "load from query time={:?}",
                        end_load_from_query.duration_since(start_load_from_query)
                    );

                    let start_compress = std::time::Instant::now();

                    let mut out = BufWriter::new(file);
                    match webp::Encoder::from_image(&img) {
                        Ok(encoder) => {
                            let webp = encoder.encode(40f32);
                            let end_compress = std::time::Instant::now();
                            println!(
                                "compress time={:?}",
                                end_compress.duration_since(start_compress)
                            );
                            println!("compress success; url={}", url);
                            // Define and write the WebP-encoded file to a given path
                            if let Ok(size) = out.write(&webp) {
                                let mut proc_thumb_faile = false;
                                if let Some(img) = compress_image(&img, size) {
                                    if let Err(result) = save_thumbnail(&img, &file_path_thumbnail)
                                    {
                                        println!("save thumbnail err={}", result);
                                        proc_thumb_faile = true;
                                    }
                                } else {
                                    proc_thumb_faile = true;
                                }
                                if proc_thumb_faile {
                                    url_thumb = url.clone();
                                }

                                let (width, height) = img.dimensions();

                                let data = Data {
                                    name,
                                    url,
                                    thumb: url_thumb,
                                    time: chrono::Local::now().timestamp(),
                                    size: size as i64,
                                    width: width as i64,
                                    height: height as i64,
                                };
                                return Data::add(data).json();
                            }
                        }
                        Err(e) => {
                            println!("compress error {}", e);
                        }
                    };
                }
                Err(e) => {
                    println!("save err {}", e);
                }
            }
        }
    }

    ReqResult {
        success: false,
        msg: "上传失败".to_string(),
        code: -1,
        data: "".to_string(),
    }
    .json()
}

// 缩小图片
fn compress_image(img: &DynamicImage, size: usize) -> Option<DynamicImage> {
    let thumbnail_size = 300;
    let (width, height) = img.dimensions();

    if width <= thumbnail_size || height <= thumbnail_size || size <= 150_000 {
        return None;
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
    // 将原始尺寸的图片缩小到指定尺寸
    Some(img.resize(nwidth, nheight, image::imageops::FilterType::Nearest))
}

// 生成缩略图
fn save_thumbnail(img: &DynamicImage, file_path_thumbnail: &str) -> Result<(), String> {
    let start_save_thumbnail = std::time::Instant::now();
    match webp::Encoder::from_image(img) {
        Err(err) => return Err(err.to_string()),
        Ok(encoder) => {
            let webp = encoder.encode(40f32);
            match File::create(file_path_thumbnail) {
                Err(err) => return Err(err.to_string()),
                Ok(file) => match BufWriter::new(file).write(&webp) {
                    Err(err) => return Err(err.to_string()),
                    Ok(size) => {
                        let end_save_thumbnail = std::time::Instant::now();
                        println!(
                            "save thumbnail time={:?}, file_size={}, file_path={}",
                            end_save_thumbnail.duration_since(start_save_thumbnail),
                            size,
                            file_path_thumbnail
                        );
                    }
                },
            }
        }
    };

    Ok(())
}

fn delete(params: HashMap<String, String>) -> String {
    println!("{params:?}");
    if let Some(thumb_url) = params.get("url") {
        let mut url = thumb_url.replace("./res", "./web/res");
        if url.contains("_thumb") {
            let _ = remove_file(Path::new(&url)); // 先删掉缩略图
            url = url.replace("_thumb", "");
        }

        let save_path = Path::new(&url);
        match remove_file(save_path) {
            Ok(_) => Data::del(thumb_url.to_string()).json(),
            Err(e) => ReqResult {
                success: false,
                code: -1,
                msg: e.to_string(),
                data: url.to_string(),
            }
            .json(),
        }
    } else {
        ReqResult {
            success: false,
            code: -2,
            msg: "请指定正确的url".to_string(),
            data: "".to_string(),
        }
        .json()
    }
}

fn delete_all(params: HashMap<String, String>) -> String {
    println!("{params:?}");
    if let Some(thumb_url) = params.get("url") {
        let thumb_url = thumb_url.replace("[", "");
        let thumb_url = thumb_url.replace("]", "");
        let thumb_url = thumb_url.replace("\"", "");
        let thumb_url_split = thumb_url.split(",");
        let mut results = vec![];
        let mut success = true;
        for thumb_url in thumb_url_split {
            let mut url = thumb_url.replace("./res", "./web/res");
            if url.contains("_thumb") {
                let _ = remove_file(Path::new(&url)); // 先删掉缩略图
                url = url.replace("_thumb", "");
            }

            let save_path = Path::new(&url);
            let result = match remove_file(save_path) {
                Ok(_) => Data::del(thumb_url.to_string()).json(),
                Err(e) => {
                    success = false;
                    ReqResult {
                        success: false,
                        code: -1,
                        msg: e.to_string(),
                        data: url.to_string(),
                    }
                    .json()
                }
            };
            results.push(result);
        }

        if success {
            ReqResult {
                success,
                code: 0,
                msg: "删除成功".to_string(),
                data: results,
            }
            .json()
        } else {
            ReqResult {
                success: false,
                code: -1,
                msg: "删除失败，可刷新页面重试".to_string(),
                data: results,
            }
            .json()
        }
    } else {
        ReqResult {
            success: false,
            code: -2,
            msg: "请指定正确的url".to_string(),
            data: "".to_string(),
        }
        .json()
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

    pub fn del(url: String) -> ReqResult<String> {
        let del = |conn: Connection, thumb_url: String| {
            let (success, msg, code, rows) = sqlite::fmt_result(
                conn.execute("DELETE FROM picture WHERE thumb = ?1", params![thumb_url]),
            );
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

mod utils {
    use std::path::Path;

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

    // pub fn get_value<T: ToString + FromStr>(params: HashMap<String, String>, key: &String, value: T) -> T {
    //     use std::{collections::HashMap, path::Path, str::FromStr};
    //     let tmp = params.get(key).or(Some(&value.to_string())).unwrap();
    //     let value: T = tmp.parse().or::<FromStr::Err>(Ok(value)).unwrap();
    //     value
    // }
}
