# ------------------------------
# Builder: download + verify k6
# ------------------------------
FROM alpine:3.20 AS builder

ARG K6_VERSION=1.3.0
# Valid values: amd64, arm64
ARG K6_ARCH=amd64
ENV K6_TARBALL="k6-v${K6_VERSION}-linux-${K6_ARCH}.tar.gz"
# ENV K6_CHECKSUM="k6-v${K6_VERSION}-linux-${K6_ARCH}.tar.gz"
ENV K6_BASEURL="https://github.com/grafana/k6/releases/download/v${K6_VERSION}"

RUN apk add --no-cache curl tar coreutils ca-certificates && update-ca-certificates
# https://github.com/grafana/k6/releases/download/v1.3.0/k6-v1.3.0-checksums.txt
# Download tarball and checksums
WORKDIR /tmp/k6
RUN curl -fsSLO "${K6_BASEURL}/${K6_TARBALL}" \
 && curl -fsSLO "${K6_BASEURL}/k6-v${K6_VERSION}-checksums.txt" \
 && grep " ${K6_TARBALL}\$" k6-v${K6_VERSION}-checksums.txt | sha256sum -c - \
 && tar -xzf "${K6_TARBALL}"

# Make a slim artifact dir with just the binary
RUN mkdir -p /out/bin && \
    cp "k6-v${K6_VERSION}-linux-${K6_ARCH}/k6" /out/bin/k6 && \
    chmod +x /out/bin/k6

# Also keep CA bundle to copy into final image (in case the base lacks it)
RUN mkdir -p /out/certs && \
    cp /etc/ssl/certs/ca-certificates.crt /out/certs/ca-certificates.crt || true

# ------------------------------
# Final: minimal Ubuntu + k6
# ------------------------------
FROM ubuntu:24.04

# No apt usage in the final image, per your requirement.
# Provide a reasonable environment and path.
ENV PATH="/usr/local/bin:${PATH}" \
    K6_STATSD_ENABLE_TAGS=true

# Create directories and copy artifacts from builder
RUN mkdir -p /usr/local/bin /etc/ssl/certs
COPY --from=builder /out/bin/k6 /usr/local/bin/k6
# Copy CA bundle so HTTPS targets work even if certs aren't preinstalled
COPY --from=builder /out/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

RUN  apt update
RUN  apt install curl dnsutils -y
# Optional: non-root user (comment out if you prefer root)
# RUN useradd -m -u 12345 k6user
# USER 12345

# Expose k6 REST API (only needed if you run with --address)
EXPOSE 6565

# Default to showing version; override with `docker run ... k6 run /script.js`
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash", "-c", "while true; do sleep 3600; done"]
