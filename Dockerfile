
FROM rust:1.61 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/my-rust-app .

EXPOSE 3000

CMD ["./be-minanonihongo-romanji"]
