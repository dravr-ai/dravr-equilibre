# ABOUTME: Multi-stage Docker build for dravr-equilibre-server and dravr-equilibre-mcp binaries
# ABOUTME: Minimal runtime image for health domain model server

FROM rust:1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release -p dravr-equilibre-server -p dravr-equilibre-mcp

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash equilibre

COPY --from=builder /build/target/release/dravr-equilibre-server /usr/local/bin/
COPY --from=builder /build/target/release/dravr-equilibre-mcp /usr/local/bin/

USER equilibre
WORKDIR /home/equilibre

EXPOSE 3000
ENTRYPOINT ["dravr-equilibre-server"]
CMD ["--host", "0.0.0.0"]
