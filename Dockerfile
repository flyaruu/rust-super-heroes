FROM rust:1.99-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY database ./database
COPY lib ./lib
COPY services ./services

RUN cargo build --release --workspace --bins

FROM gcr.io/distroless/cc-debian12 AS rest-heroes
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=builder /app/target/release/rest-heroes /rest-heroes
CMD ["/rest-heroes"]

FROM gcr.io/distroless/cc-debian12 AS rest-villains
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=builder /app/target/release/rest-villains /rest-villains
CMD ["/rest-villains"]

FROM gcr.io/distroless/cc-debian12 AS grpc-locations
ENV RUST_LOG=info
EXPOSE 50051
COPY --from=builder /app/target/release/grpc-locations /grpc-locations
CMD ["/grpc-locations"]

FROM gcr.io/distroless/cc-debian12 AS rest-fights
ENV RUST_LOG=info
EXPOSE 8082
COPY --from=builder /app/target/release/rest-fights /rest-fights
CMD ["/rest-fights"]
