FROM gcr.io/distroless/cc-debian12
ENV RUST_LOG=info
EXPOSE 8000

COPY binary/rest-villains /rest-villains
CMD ["/rest-villains"]
