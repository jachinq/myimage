cargo build --release --target x86_64-unknown-linux-musl
mkdir app
cp target/x86_64-unknown-linux-musl/release/myimage app/
cp -r web app/
cd app
docker build -t myimage .
cd ..