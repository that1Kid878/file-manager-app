#-----BUILD-----
FROM rust:slim AS builder

WORKDIR /backend

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
RUN cargo build --release

#-----PROD-----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/liists/*

COPY --from=builder /backend/target/release/file-manager-backend /usr/local/bin/file-manager-backend

EXPOSE 8080

CMD ["/usr/local/bin/file-manager-backend"]
