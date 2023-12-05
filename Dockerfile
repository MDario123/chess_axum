# Build stage
FROM docker.uclv.cu/rust:1.73.0-slim-buster as builder
RUN cargo new --bin rust-and-docker
WORKDIR ./app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

# Run stage
# FROM docker.uclv.cu/debian:buster-slim
FROM docker.uclv.cu/alpine:3.18
WORKDIR /app
COPY --from=builder /app/target/release/planner .
CMD ["/app/planner"]
