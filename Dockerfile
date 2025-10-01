# ---- Build stage ----
FROM rust:1-bookworm AS builder
WORKDIR /app

# cache deps
COPY Cargo.toml Cargo.lock build.rs ./
COPY proto proto
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs
RUN cargo build --release || true

# real source
COPY src src
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
RUN useradd -ms /bin/bash gos
WORKDIR /home/gos
COPY --from=builder /app/target/release/gos /usr/local/bin/gos
ENV RUST_LOG=info
EXPOSE 50051
USER gos
CMD ["gos"]
