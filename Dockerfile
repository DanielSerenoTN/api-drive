FROM rust:1.73 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/api_drive /usr/local/bin/api_drive

EXPOSE 8080

CMD ["api_drive"]
