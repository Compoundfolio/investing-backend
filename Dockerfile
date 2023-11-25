FROM rust AS builder
WORKDIR /usr/src/investing-backend-rs
COPY . .
RUN cargo install --path . 

FROM debian:bookworm-slim
WORKDIR /opt/investing-backend-rs
RUN apt-get update && apt-get install -y postgresql-client ca-certificates
COPY . .
COPY --from=builder /usr/local/cargo/bin/investing-backend-rs ./
CMD ["./investing-backend-rs"]
