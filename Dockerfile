FROM rust:1-slim

WORKDIR /app

COPY src/ /app/src
COPY Cargo.toml Cargo.lock /app/

EXPOSE 8080
RUN cargo build --release

ENTRYPOINT [ "/app/target/release/docker-api-proxy" ]