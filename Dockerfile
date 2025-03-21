FROM rust:1.85 AS build
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
WORKDIR /build
COPY . .
RUN cargo build --bins --locked --release

FROM gcr.io/distroless/cc-debian12 AS rest-heroes
COPY --from=build /build/target/release/rest-heroes /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/rest-heroes"]

FROM gcr.io/distroless/cc-debian12 AS rest-villains 
COPY --from=build /build/target/release/rest-villains /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/rest-villains"]


FROM gcr.io/distroless/cc-debian12 AS grpc-locations
COPY --from=build /build/target/release/grpc-locations /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/grpc-locations"]

FROM gcr.io/distroless/cc-debian12 AS rest-fights 
COPY --from=build /build/target/release/rest-fights /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/rest-fights"]
