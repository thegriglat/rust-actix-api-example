### Build image
FROM alpine as rust-build
WORKDIR /app

RUN apk add rust cargo --no-cache

FROM rust-build as app-build
RUN apk add --no-cache libpq libpq-dev
COPY . .
RUN cargo build --release


### Runner image
FROM alpine as api
WORKDIR /app
RUN apk add --no-cache libgcc libpq
COPY --from=app-build /app/target/release/rest-api .

CMD ["/app/rest-api"]


### migrations image
FROM rust-build as diesel
WORKDIR /app
RUN apk add --no-cache libpq libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
CMD ["/root/.cargo/bin/diesel", "setup"]