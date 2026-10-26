FROM rust:1.85-alpine AS builder

# Системные пакеты для компиляции Си-зависимостей rdkafka
RUN apk add --no-cache \
    musl-dev cmake make g++ git openssl-dev openssl-libs-static \
    cyrus-sasl-dev curl-dev zlib-dev zlib-static

WORKDIR /app

# 1. Генерируем пустышки
RUN cargo new --bin src/services/ingestion && \
    cargo new --lib src/kernel

# 2. Копируем файлы манифестов 
COPY Cargo.toml ./
COPY src/kernel/Cargo.toml ./src/kernel/
COPY src/services/ingestion/Cargo.toml ./src/services/ingestion/

# Собираем внешние библиотеки
RUN cargo build --release
RUN rm -rf src/kernel/src src/services/ingestion/src target/release/deps/ingestion_service*

# 3. Копируем исходный код
COPY src/kernel ./src/kernel
COPY src/services/ingestion ./src/services/ingestion

# Финальный билд
RUN cargo build --release -p ingestion_service


FROM alpine:3.19
RUN apk add --no-cache ca-certificates


COPY --from=builder /app/target/release/ingestion_service /usr/local/bin/ingestion_service

EXPOSE 8080/tcp
EXPOSE 8081/udp

CMD ["/usr/local/bin/ingestion_service"]
