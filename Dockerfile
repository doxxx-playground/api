# Rust 빌드 이미지
FROM rust:1.82 as builder

WORKDIR /app
COPY . .

# 의존성 캐싱
RUN cargo build --release

# 런타임 이미지
FROM debian:buster-slim

WORKDIR /app

# 빌드된 바이너리 복사
COPY --from=builder /app/target/release/api .

# PostgreSQL 연결을 위해 OpenSSL 설치
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# 실행
CMD ["./api"]
