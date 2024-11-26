use std::collections::HashMap;

use image::GenericImageView;

use crate::{proc_result, utils, Data, ReqResult, THUMB_QUALITY};

use super::parse_body;

pub fn get_api_data(url: &str, body: &str) -> String {
    let params = parse_body(&body);
    match &url.replace("api/", "") as &str {
        "/test" => test(params),
        "/getAll" => get_all(params),
        "/upload" => upload(params),
        "/delete" => delete(params),
        "/deleteAll" => delete_all(params),
        _ => ReqResult::error("找不到方法", url).json(),
    }
}

fn test(params: HashMap<String, String>) -> String {
    println!("匹配到了 /test. {:?}", params);
    ReqResult::success("ping success", "ping").json()
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
