ARG BASE_IMAGE=rust:1.75.0

FROM $BASE_IMAGE as builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/api_drive /

EXPOSE 8080
CMD ["./api_drive"]
