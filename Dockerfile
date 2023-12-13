# Build stage
FROM rust:1.73.0-slim-buster as builder
WORKDIR /app
COPY . .
ARG DATABASE_URL=""
ENV DATABASE_URL=${DATABASE_URL}
RUN cargo build --release

# Run stage
FROM debian:buster-slim
# FROM docker.uclv.cu/alpine:3.18
WORKDIR /app
COPY --from=builder /app/target/release/chess_uclv /app
CMD ["/app/chess_uclv"]
