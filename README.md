# rust-link-shortener
Simple REST-api written in Rust using [Actix](https://actix.rs) and [Diesel](https://diesel.rs)

It was started as education pet project, hope it will be useful also for you.


# How to start developing

1. Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
cargo run
```

# To start API service in Docker

1. Build images

```bash
docker-compose build
```

3. Start API server

```bash
docker-compose up
```

4. Test your API

```bash
$ curl -X POST -H "Content-Type: application/json" http://localhost:8080 -d '{"url": "https://google.com"}'
{"url":"4vihsq"}

$ curl http://localhost:8080/4vihsq
{"url":"https://google.com"}
```