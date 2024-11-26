use std::{collections::HashMap, path::Path, str::FromStr};

use crate::{ReqResult, RESOURCE_DIR};

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
/// 普通文件: RESOURCE_DIR[/appid]/year_month
/// 缩略图: RESOURCE_DIR[/appid]/thumb/year_month
/// 普通文件url: RESOURCE_DIR[/appid]/year_month/uuid8bit.ext
/// 缩略图url: RESOURCE_DIR[/appid]/thumb/year_month/uuid8bit.ext
pub fn get_file_path_and_url(
    is_thumb: bool,
    appid: Option<&String>,
    uuid: &str,
    ext: &str,
) -> (String, String) {
    let mut directory = String::new();
    directory.push_str(RESOURCE_DIR);
    if let Some(appid) = appid {
        directory.push_str("/");
        directory.push_str(appid);
    }
    if is_thumb {
        directory.push_str("/thumb");
    }
    let fmt = "%Y%m";
    let year_month = chrono::Local::now().format(fmt).to_string();
    directory.push_str("/");
    directory.push_str(&year_month);
    // directory.push_str("/");
    // 此时 path 应该是 /RESOURCE_DIR[/appid][/thumb]/year_month
    // 对应的就是文件夹，检查一下是否已存在并自动创建
    check_dir_and_create(&directory);

    let filename = format!("{}.{}", uuid, ext);

    let file_path = format!("{}/{}", directory, filename);
    let url = format!("{}/{}", directory, filename);
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
