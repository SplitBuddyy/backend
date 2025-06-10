FROM rust:1-alpine AS chef
RUN rustup install 1.81.0
RUN rustup component add cargo clippy rust-docs rust-std rustc rustfmt

# Use apk for package management in Alpine
RUN apk add --no-cache build-base libressl-dev curl
RUN cargo install cargo-chef

FROM chef AS planner

WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --release

# Build application
COPY . .
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release


FROM python:3.9.18-slim-bookworm AS final

WORKDIR /

COPY --from=builder /app/target/release/trip_split /usr/local/bin/prover


EXPOSE 3000

ENTRYPOINT [ "prover" ]