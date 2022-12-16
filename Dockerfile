FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 as chef

ENV SQLX_OFFLINE true
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner

COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
RUN cargo build --release --bin zero2prod

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

# WIP
# FROM rust:1.65.0 AS builder

# ENV SQLX_OFFLINE true
# ENV RUST_BACKTRACE 1

# WORKDIR /app

# COPY . .

# RUN apt update \
#   && apt upgrade -y \
#   && apt install lld clang -y \
#   && rustup target add x86_64-unknown-linux-musl \
#   && cargo build --release \
#   && cargo install --target x86_64-unknown-linux-musl --path .

# FROM scratch AS final

# ENV APP_ENVIRONMENT production

# COPY --from=builder /app/zero2prod .

# EXPOSE 8000
# USER 1000

# ENTRYPOINT [ "./zero2prod" ]
