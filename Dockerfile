FROM rust:1.66.0 AS builder

ENV SQLX_OFFLINE true
WORKDIR /app

COPY . .

RUN apt update \
  && apt install lld clang -y \
  && cargo build --release --bin zero2prod 

FROM debian:bullseye-slim AS final

ENV APP_ENVIRONMENT production
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

EXPOSE 8000

ENTRYPOINT ["./zero2prod"]
