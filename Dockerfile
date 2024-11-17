# Builder stage
FROM rust:1.82.0 AS builder
WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletterapi newsletterapi
COPY configuration configuration
ENV APP_ENVIRONMENT production
EXPOSE 8000
ENTRYPOINT ["./newsletterapi"]

