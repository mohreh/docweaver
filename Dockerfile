FROM rust:slim-bullseye AS builder
WORKDIR /app

RUN apt-get update
RUN apt-get install --yes --no-install-recommends \
    build-essential \
    ninja-build \
    libssl-dev \
    pkg-config \
    ca-certificates \
    nodejs \
    npm \
    openssl

RUN update-ca-certificates

RUN npm i -g tailwindcss

COPY . .
ENV APP_ENVIRONMENT production
RUN cargo build --release
RUN ls

FROM debian:bullseye-slim AS runtime

RUN apt-get update
RUN apt-get install openssl

WORKDIR /app

COPY --from=builder /app/config.yml config.yml
COPY --from=builder /app/assets assets
COPY --from=builder /app/docs docs
COPY --from=builder /app/templates templates
COPY --from=builder /app/target/release/docweaver docweaver

ENV APP_ENVIRONMENT production

EXPOSE 8000
ENTRYPOINT ["./docweaver"]

# FROM rust:alpine as builder
# WORKDIR /app
#
# RUN apk add --no-cache --update build-base openssl-dev nodejs npm
#
# COPY . .
#
# RUN npm i -g tailwindcss
#
# RUN cargo build --release --bin docweaver --target x86_64-unknown-linux-musl
#
# FROM alpine:latest AS runtime
# WORKDIR /app
#
# RUN apk add --no-cache openssl ca-certificates
# RUN addgroup -S app && adduser -S app -G app
#
# COPY --from=builder /app/config.yml config.yml
# COPY --from=builder /app/assets assets
# COPY --from=builder /app/docs docs
# COPY --from=builder /app/templates templates
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/docweaver docweaver
#
# RUN chown -R app:app /app
# USER app
#
# EXPOSE 5000
# ENTRYPOINT ["./docweaver"]
