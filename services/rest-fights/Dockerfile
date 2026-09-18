FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
EXPOSE 8000

COPY binary/rest-fights /rest-fights
CMD ["/rest-fights"]
