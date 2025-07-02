# Stage 1: Build the application
FROM rust:1-slim as builder

WORKDIR /usr/src/versionwatch

# First, copy over the manifests to cache dependencies
COPY Cargo.toml Cargo.lock ./
# Create dummy files for each crate to build the dependency tree
RUN mkdir -p crates/versionwatch-cli/src \
    && echo "fn main() {}" > crates/versionwatch-cli/src/main.rs \
    && mkdir -p crates/versionwatch-core/src \
    && echo "pub struct Dummy;" > crates/versionwatch-core/src/lib.rs \
    && mkdir -p crates/versionwatch-collect/src \
    && echo "pub struct Dummy;" > crates/versionwatch-collect/src/lib.rs \
    && mkdir -p crates/versionwatch-config/src \
    && echo "pub struct Dummy;" > crates/versionwatch-config/src/lib.rs \
    && mkdir -p crates/versionwatch-db/src \
    && echo "pub struct Dummy;" > crates/versionwatch-db/src/lib.rs

# Build only the dependencies to leverage Docker layer caching
RUN cargo build --release

# Now copy the actual source code
COPY . .

# Build the application, which will be faster as deps are cached
RUN cargo build --release

# Stage 2: Create the final, minimal image
FROM debian:slim-bookworm as final

WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/versionwatch/target/release/versionwatch /usr/local/bin/versionwatch

# Optional: Copy configuration files if they should be baked into the image
# COPY --from=builder /usr/src/versionwatch/config /etc/versionwatch/config

# Run the binary
ENTRYPOINT ["/usr/local/bin/versionwatch"] 