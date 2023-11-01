FROM rust AS builder
WORKDIR /usr/src/investing-backend-rs
COPY . .
RUN cargo install --path . 

FROM debian:bookworm-slim
ARG RUN_MODE=prod
WORKDIR /opt/investing-backend-rs
RUN apt-get update && apt-get install -y postgresql-client ca-certificates
COPY . .
COPY --from=builder /usr/local/cargo/bin/investing-backend-rs ./
EXPOSE 8080
ENV RUN_MODE=${RUN_MODE}
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
CMD ["./investing-backend-rs"]
