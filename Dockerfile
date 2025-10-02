FROM rust:alpine as builder

WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/lodelix /lodelix

EXPOSE 3000

CMD ["/lodelix"]