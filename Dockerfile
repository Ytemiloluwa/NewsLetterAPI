FROM rust:1.82.0
WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
EXPOSE 8000
ENTRYPOINT ["./target/release/newsletterapi"]
