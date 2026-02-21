# Build Stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app
COPY . .

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/radio-broadcaster /app/radio-station

# Create entrypoint for easy startup
ENTRYPOINT ["/app/radio-station"]
