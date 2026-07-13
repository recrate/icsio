FROM rust:1.97.0-alpine AS builder

ARG BIN_NAME=icsio

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release && cp target/release/${BIN_NAME} /app/server

FROM scratch AS runtime

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/server /server

EXPOSE 3000
USER 1000:1000
ENTRYPOINT ["/server"]