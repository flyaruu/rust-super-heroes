FROM rust:1.91 AS workspace-build
RUN apt-get update \
 && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
COPY . .
RUN pwd && ls -la && ls -la services && ls -la lib
RUN cargo build --workspace --release

FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
COPY --from=workspace-build /workspace/target/release/rest-heroes /
COPY --from=workspace-build /workspace/target/release/rest-villains /
COPY --from=workspace-build /workspace/target/release/grpc-locations /
COPY --from=workspace-build /workspace/target/release/rest-fights /
CMD ["/rest-heroes"]
