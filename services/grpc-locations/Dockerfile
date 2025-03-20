FROM rust:1.85 AS build
WORKDIR /build
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

COPY Cargo.lock Cargo.toml build.rs src proto ./
COPY src src
COPY proto proto
RUN cargo build --bins --locked --release

FROM gcr.io/distroless/cc-debian12 AS final
COPY --from=build /build/target/release/grpc-locations /
ENV HTTP_PORT=80
ENV RUST_LOG=info
EXPOSE 80
CMD ["/grpc-locations"]