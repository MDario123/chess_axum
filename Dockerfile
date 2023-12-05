# Build stage
FROM rust:1.73.0-slim-buster as builder
RUN cargo new --bin rust-and-docker
WORKDIR ./app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
ARG DATABASE_URL=""
ENV DATABASE_URL=${DATABASE_URL}
RUN cargo build --release

# Run stage
# FROM docker.uclv.cu/debian:buster-slim
FROM alpine:3.18
WORKDIR /app
COPY --from=builder /app/target/release/chess_uclv .
CMD ["/app/chess_uclv"]
