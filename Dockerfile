FROM rust AS builder
WORKDIR /usr/src/investing-backend-rs
COPY . .
RUN cargo install --path . 

FROM debian:bullseye-slim
WORKDIR /opt/investing-backend-rs
RUN apt-get update && apt-get install -y postgresql-client
COPY . .
COPY --from=builder /usr/local/cargo/bin/investing-backend-rs ./
EXPOSE 4430
CMD ["./investing-backend-rs"]
