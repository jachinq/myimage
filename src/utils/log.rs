use std::time::Instant;

pub fn log(level: &str, info: &str) {
    let now = chrono::Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S");
    println!(
        "<{}>[{}] {{i={};}} ",
        level,
        datetime,
        info,
    );
}
pub fn log_time_used(start_time: Instant, info: &str) {
    let end_time = Instant::now();
    let now = chrono::Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S");
    println!(
        "<Info>[{}] {{cost={:?};i={};}}",
        datetime,
        end_time.duration_since(start_time),
        info,
    );
}

pub fn log_req(start: Instant, url: &str, ip: &str) {
    let app_name = "myimage";
    // 获取当前进程pid
    let pid = std::process::id();
    let now = chrono::Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S");
    // <app_name:pid>[datetime] {flow} {req;t=1;i=ip;u=url;}
    let end = Instant::now();
    let duration = end.duration_since(start);
    // 耗时请求重点关注：
    let focus = if duration.as_millis() > 1000 {
        "slow;"
    } else {
        ""
    };

    println!("<{app_name}:{pid}>[{datetime}] {{{focus}t={duration:?};i={ip};u={url};}}");
}
