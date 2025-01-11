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
