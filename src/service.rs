use anyhow::{anyhow, Error, Result};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::{fs::remove_file, path::Path};

use crate::{utils::Matcher, ReqResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageData {
    pub list: Vec<Data>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub url: String,
    pub thumb: String,
    pub time: i64,
    pub size: i64,
    pub width: i64,
    pub height: i64,
}

impl Data {
    fn map_to_data(row: &rusqlite::Row) -> Result<Self> {
        Ok(Self {
            name: row.get(0)?,
            url: row.get(1)?,
            thumb: row.get(2)?,
            time: row.get(3)?,
            size: row.get(4)?,
            width: row.get(5)?,
            height: row.get(6)?,
        })
    }

    pub fn add(data: Self) -> Result<Self, Error> {
        // 成功连接数据库后执行业务逻辑
        let conn = sqlite::connect()?;

        // 业务逻辑
        let fields = "name,url,thumb,time,size,width,height";
        let values = sqlite::turn_values(fields.to_string());

        let (success, msg, _code, rows) = sqlite::fmt_result(conn.execute(
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
        Ok(data)
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

    pub fn get_list(mut matcher: Matcher) -> Result<PageData, Error> {
        matcher.table("picture");
        matcher.fields(&["*"]);
        let sql = matcher.build();
        let count_sql = matcher.build_count();

        // println!("sql==={sql} count_sql={count_sql}");
        let conn = sqlite::connect()?;
        let mut stmt = conn.prepare(&sql)?;

        let iter = stmt.query_map(params![], |row| Ok(Self::map_to_data(row)))?;

        let total = conn.query_row(&count_sql, [], |row| {
            // println!("row={:?}", row);
            let total = row.get_ref(0)?.as_i64()?;
            Ok(total)
        })?;

        let mut list = Vec::new();

        for some in iter {
            list.push(some??);
        }

        Ok(PageData { list, total })
    }

    pub fn del(url: String, is_thumb: bool) -> Result<String, Error> {
        let conn = sqlite::connect()?;
        let sql = if is_thumb {
            "DELETE FROM picture WHERE thumb = ?1"
        } else {
            "DELETE FROM picture WHERE url = ?1"
        };
        let (success, msg, _code, rows) = sqlite::fmt_result(conn.execute(sql, params![url]));
        println!(
            "del picture end; rows={:?} url={:?} success={} msg={}",
            rows, url, success, msg
        );

        println!(
            "del picture end; rows={:?} url={:?} success={} msg={}",
            rows, url, rows, msg
        );
        if success {
            let msg = "删除成功, 行数=".to_string() + &rows.to_string();
            Ok(msg.to_string())
        } else {
            Err(anyhow!(msg))
        }
    }

    pub fn delete_single(url_arg: &str) -> Result<String, Error> {
        if url_arg.is_empty() {
            return Err(anyhow!("url不能为空"));
        }
        let is_thumn = url_arg.contains("thumb");
        let data = Data::get_by_url(url_arg, is_thumn);
        if data.is_none() {
            return Err(anyhow!("找不到需要删除的图片"));
        }

        let data = data.unwrap();
        let url = data.url;
        let thumb = data.thumb;

        let ok = remove_file(Path::new(&thumb)); // 先删掉缩略图
        if ok.is_err() {
            println!("delete thumb err={}, path={}", ok.unwrap_err(), thumb);
            return Err(anyhow!("删除缩略图失败"));
        }

        let ok = remove_file(Path::new(&url));
        if ok.is_err() {
            println!("delete img err={}, path={}", ok.unwrap_err(), url);
            return Err(anyhow!("删除图片失败"));
        }
        Data::del(url_arg.to_string(), is_thumn)
    }
}

pub mod sqlite {
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

pub fn proc_result<T: Serialize>(result: Result<T, Error>) -> ReqResult<Option<T>> {
    match result {
        Ok(data) => ReqResult::success("success", Some(data)),
        Err(e) => ReqResult::error(&e.to_string(), None),
    }
}
