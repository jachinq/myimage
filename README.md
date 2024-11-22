# myimage

[中文](README_zh.md)

## Introduction

This is an image hosting website.

Use Rust to build it.

dependencies:

- serde + serde_json + serde_derive (for JSON serialization/deserialization)
- tiny_http + urlencoded (for HTTP server)
- rustsqlite + uuid + chrono (for database)
- image + image_base64 + webp (for image processing)

Frontend: Vanlla JavaScript + HTML + CSS

## Data

Index database will be created at `data/data.db`.

Picture files will be stored at `web/res/pictures`, every picture will have a unique UUID as its filename, and it will be generated a thumbnail when uploaded. The thumbnail is used for displaying in the frontend of list page.

## Usage

## Development

git clone the repository.

```
cargo run
```

This will start the server at http://localhost:8080.


## deploy

```
cargo build --release --target x86_64-unknown-linux-musl
```

Then copy the binary and the web directory to the `app` directory.

```
mkdir app
cp target/x86_64-unknown-linux-musl/release/myimage app/
cp -r web app/
```

Start the server:

```
cd app
./myimage
```

## Deploy to Docker

Run with docker, the project file strucure is:

```
myimage/
├── Dockerfile
├── app
│   ├── myimage
│   └── web
└── docker-compose.yml
```

At first, you need to use `docker` to build the image.

```
docker build -t myimage .
```

Run the container with docker-compose.

```
docker compose up -d
```

