FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
EXPOSE 50051

COPY binary/grpc-locations /grpc-locations
CMD ["/grpc-locations"]
