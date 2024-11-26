use std::{collections::HashMap, path::Path, str::FromStr};

// const STATIC_DIR: &str = "./web"; // 指定你的静态文件目录
use crate::{ReqResult, STATIC_DIR};

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

use serde::Serialize;
use serde_json::Value;
fn extract_known_fields(
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
