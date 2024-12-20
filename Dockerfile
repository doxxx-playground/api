# 빌드 스테이지
FROM rust:1.82-slim AS builder

# 필요한 패키지 설치
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 작업 디렉토리 설정
WORKDIR /usr/src/app

# diesel CLI 설치
RUN cargo install diesel_cli --no-default-features --features postgres

# 의존성 파일 복사 및 캐시 활용
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# 소스 및 마이그레이션 파일 복사
COPY . .
RUN cargo build --release

# 런타임 스테이지
FROM debian:bookworm-slim

# 필요한 런타임 의존성 설치
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 비특권 사용자 생성
RUN useradd -r -s /bin/false appuser

# 작업 디렉토리 설정
WORKDIR /app

# 빌드된 바이너리와 마이그레이션 파일 복사
COPY --from=builder /usr/src/app/target/release/api /app/api
COPY --from=builder /usr/src/app/migrations /app/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# 권한 설정
RUN chown -R appuser:appuser /app
USER appuser

# 환경 변수 설정
ENV DATABASE_URL=""
ENV HOST="0.0.0.0"
ENV PORT="8080"
ENV RUST_LOG="info"

# 헬스체크 설정
HEALTHCHECK --interval=30s --timeout=3s \
    CMD ["/app/api", "--health-check"]

# 포트 노출
EXPOSE 8080

# 애플리케이션 실행 명령어
CMD ["/app/api"]
