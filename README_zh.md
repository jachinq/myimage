# myimage

[English](README.md)

## 介绍

这是一个图片托管网站。

使用 Rust 来构建它。

依赖项：

- serde + serde_json + serde_derive（用于 JSON 序列化/反序列化）
- tiny_http + urlencoded（用于 HTTP 服务器）
- rustsqlite + uuid + chrono（用于数据库）
- image + image_base64 + webp（用于图像处理）

前端：原生 JavaScript + HTML + CSS

## 数据

索引数据库将创建在 `data/data.db`。

图片文件将存储在 `web/res/pictures`，每张图片将有一个唯一的 UUID 作为其文件名，并在上传时生成缩略图。缩略图用于在列表页面的前端显示。

## 使用

## 开发

首先克隆仓库。

```
cargo run
```

这将启动服务器，地址为 http://localhost:8080。

## 部署

```
cargo build --release --target x86_64-unknown-linux-musl
```

然后将二进制文件和 web 目录复制到 `app` 目录。

```
mkdir app
cp target/x86_64-unknown-linux-musl/release/myimage app/
cp -r web app/
```

启动服务器：

```
cd app
./myimage
```

## 部署到 Docker

使用 docker 运行，项目文件结构如下：

```
myimage/
├── Dockerfile
├── app
│   ├── myimage
│   └── web
└── docker-compose.yml
```

首先，您需要使用 `docker` 构建镜像。

```
docker build -t myimage .
```

使用 docker-compose 运行容器

```
docker compose up -d
```

