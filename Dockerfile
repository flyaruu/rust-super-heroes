FROM rust:1.85 AS build
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
WORKDIR /build

# Step 1: Copy workspace Cargo files to cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY lib/superhero-types lib/superhero-types
COPY services/rest-heroes/Cargo.toml services/rest-heroes/Cargo.toml
COPY services/rest-villains/Cargo.toml services/rest-villains/Cargo.toml
COPY services/grpc-locations/Cargo.toml services/grpc-locations/Cargo.toml
COPY services/rest-fights/Cargo.toml services/rest-fights/Cargo.toml

# Step 2: Create placeholder files for dependency resolution
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir -p services/rest-heroes/src && echo "fn main() {}" > services/rest-heroes/src/main.rs
RUN mkdir -p services/rest-villains/src && echo "fn main() {}" > services/rest-villains/src/main.rs
RUN mkdir -p services/grpc-locations/src && echo "fn main() {}" > services/grpc-locations/src/main.rs
RUN mkdir -p services/rest-fights/src && echo "fn main() {}" > services/rest-fights/src/main.rs

# Step 3: Build dependencies
RUN cargo build --bins --release

# Step 4: Copy full source code and rebuild
COPY . .
RUN cargo build --bins --locked --release

# Base image for all services
FROM gcr.io/distroless/cc-debian12 AS base
ENV RUST_LOG=info
EXPOSE 3000

# rest-heroes service
FROM base AS rest-heroes
COPY --from=build /build/target/release/rest-heroes /
CMD ["/rest-heroes"]

# rest-villains service
FROM base AS rest-villains
COPY --from=build /build/target/release/rest-villains /
CMD ["/rest-villains"]

# grpc-locations service
FROM base AS grpc-locations
COPY --from=build /build/target/release/grpc-locations /
CMD ["/grpc-locations"]

# rest-fights service
FROM base AS rest-fights
COPY --from=build /build/target/release/rest-fights /
CMD ["/rest-fights"]
