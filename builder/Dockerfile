FROM rust:1.71-buster as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release /usr/src/myapp
CMD ["cd","usr/src/myapp"]