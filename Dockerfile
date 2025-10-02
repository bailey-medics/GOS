FROM rust:1.90.0-alpine3.22
WORKDIR /app

# Install system dependencies needed for cargo-watch compilation and protobuf
RUN apk add --no-cache musl-dev gcc protobuf-dev

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Copy dependency files for initial caching (without source)
COPY Cargo.toml Cargo.lock build.rs ./
COPY proto proto

# Create minimal dummy source just to cache dependencies
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs && echo 'pub fn dummy() {}' > src/lib.rs
RUN cargo build --release
RUN rm -rf target/release/deps/gos* target/release/gos target/release/libgos*

# Don't copy real source - it will be mounted as volume

# Expose the gRPC port
EXPOSE 50051

# Use cargo watch for development
# Dependencies will be cached in the mounted target volume
CMD ["cargo", "watch", "-x", "run"]