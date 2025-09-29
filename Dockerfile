# Multi-stage build for AION-CR production deployment
FROM rust:1.75-slim-bullseye AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    cmake \
    git \
    curl \
    python3 \
    python3-pip \
    python3-dev \
    libtorch-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Python ML dependencies
RUN pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu
RUN pip3 install numpy pandas scikit-learn transformers sentence-transformers

# Set working directory
WORKDIR /usr/src/aion

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY aion-core/Cargo.toml aion-core/
COPY aion-compliance/Cargo.toml aion-compliance/
COPY aion-conflict/Cargo.toml aion-conflict/
COPY aion-db/Cargo.toml aion-db/
COPY aion-nlp/Cargo.toml aion-nlp/
COPY aion-normative/Cargo.toml aion-normative/
COPY server-workspace/Cargo.toml server-workspace/

# Create dummy source files to cache dependencies
RUN mkdir -p aion-core/src aion-compliance/src aion-conflict/src aion-db/src aion-nlp/src aion-normative/src server-workspace/src
RUN echo "fn main() {}" > aion-core/src/lib.rs
RUN echo "fn main() {}" > aion-compliance/src/lib.rs
RUN echo "fn main() {}" > aion-conflict/src/lib.rs
RUN echo "fn main() {}" > aion-db/src/lib.rs
RUN echo "fn main() {}" > aion-nlp/src/lib.rs
RUN echo "fn main() {}" > aion-normative/src/lib.rs
RUN echo "fn main() {}" > server-workspace/src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src target/release/deps/aion*

# Copy actual source code
COPY . .

# Build the application
RUN cargo build --release --all-features

# Production stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    python3 \
    python3-pip \
    curl \
    redis-tools \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Install Python ML runtime
RUN pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu
RUN pip3 install numpy pandas scikit-learn transformers sentence-transformers

# Create non-root user
RUN useradd -r -s /bin/false aion

# Create application directories
RUN mkdir -p /app/bin /app/config /app/data /app/logs /app/models
RUN chown -R aion:aion /app

# Copy binaries
COPY --from=builder /usr/src/aion/target/release/aion-server /app/bin/
COPY --from=builder /usr/src/aion/target/release/aion-cli /app/bin/

# Copy configuration files
COPY config/ /app/config/
COPY migrations/ /app/migrations/

# Copy ML models and data
COPY models/ /app/models/
COPY data/ /app/data/

# Set permissions
RUN chmod +x /app/bin/*

# Switch to non-root user
USER aion

# Set working directory
WORKDIR /app

# Environment variables
ENV RUST_LOG=info
ENV AION_CONFIG_PATH=/app/config
ENV AION_DATA_PATH=/app/data
ENV AION_MODELS_PATH=/app/models
ENV AION_LOG_PATH=/app/logs

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose ports
EXPOSE 8080 8443 9090

# Volume mounts
VOLUME ["/app/data", "/app/logs", "/app/config"]

# Default command
CMD ["/app/bin/aion-server", "--config", "/app/config/production.toml"]