### Build image
FROM alpine as rust-build
WORKDIR /app

RUN apk add rust cargo --no-cache
RUN apk add sqlite-static --no-cache

COPY . .
RUN cargo build --release


### Runner image
FROM alpine as api
WORKDIR /app
RUN apk add --no-cache libgcc
COPY --from=rust-build /app/target/release/rest-api .

CMD ["/app/rest-api"]