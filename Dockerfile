FROM rust:1.91 AS workspace-build
RUN apt-get update \
 && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
COPY . .
RUN cargo build --workspace --release

FROM gcr.io/distroless/cc-debian12 AS rest-heroes
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=workspace-build /workspace/target/release/rest-heroes /
CMD ["/rest-heroes"]

FROM gcr.io/distroless/cc-debian12 AS rest-villains
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=workspace-build /workspace/target/release/rest-villains /
CMD ["/rest-villains"]

FROM gcr.io/distroless/cc-debian12 AS grpc-locations
ENV RUST_LOG=info
EXPOSE 50051
COPY --from=workspace-build /workspace/target/release/grpc-locations /
CMD ["/grpc-locations"]

FROM gcr.io/distroless/cc-debian12 AS rest-fights
ENV RUST_LOG=info
EXPOSE 8000
COPY --from=workspace-build /workspace/target/release/rest-fights /
CMD ["/rest-fights"]
