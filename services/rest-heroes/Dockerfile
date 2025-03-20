FROM rust:1.85 AS build
WORKDIR /build

COPY Cargo.lock Cargo.toml ./
COPY src src

RUN cargo build --bins --locked --release

FROM gcr.io/distroless/cc-debian12 AS final
COPY --from=build /build/target/release/rest-heroes /
ENV HTTP_PORT=80
ENV RUST_LOG=info
EXPOSE 80
CMD ["/rest-heroes"]