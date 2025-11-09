FROM rust:1.91 AS build
WORKDIR /build

# Copy all service files
COPY . .

# Build the service
RUN cargo build --release

# Runtime image
FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
EXPOSE 8000

COPY --from=build /build/target/release/rest-heroes /
CMD ["/rest-heroes"]
