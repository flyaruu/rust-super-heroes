FROM rust:1.91 AS build
RUN apt-get update \
 && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /build

# Copy all service files
COPY . .

# Build the service
RUN cargo build --release

# Runtime image
FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
EXPOSE 50051

COPY --from=build /build/target/release/grpc-locations /
CMD ["/grpc-locations"]
