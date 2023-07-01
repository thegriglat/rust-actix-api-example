# rust-link-shortener
Simple REST-api written in Rust using [Actix](https://actix.rs) and [Diesel](https://diesel.rs)

It was started as education pet project, hope it will be useful also for you.


# How to start developing

1. Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
cargo install diesel_cli --no-default-features --features postgres

# set DB_HOST=localhost in .env file
$EDITOR .env

# run database container
docker-compose up -d db

# run migrations
diesel setup

# run Rust HTTP server (use --release to run production build)
cargo run

# Swagger will be available on http://localhost:8080/swagger-ui
```

# To start API service in Docker

```bash
# build images
docker-compose build

# Start API server
docker-compose up

# Test API
## POST method
curl -X POST -H "Content-Type: application/json" http://localhost:8080/api/links -d '{"url": "https://google.com"}'
{"url":"4vihsq"}

## GET method
curl http://localhost:8080/api/links/4vihsq
{"url":"https://google.com"}
```