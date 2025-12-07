# Builder stage
FROM docker.io/library/rust:latest as builder

# Install protobuf compiler for tonic
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/balance

# Create a dummy project to cache dependencies
RUN cargo new --bin server
WORKDIR /usr/src/balance/server
COPY Cargo.toml ./
# If you had a Cargo.lock, you would copy it here: COPY Cargo.lock ./

# Add dependencies for other binaries if needed, but we focus on server. 
# We need to make sure the Cargo.toml in the dummy matches expectations.
# Actually, 'cargo new' creates a Cargo.toml. We want to overwrite it.
# Moving up to root to match workspace structure if it was a workspace, 
# but here it is a single package.

WORKDIR /usr/src/balance
# Copy manifests
COPY Cargo.toml ./

# Create dummy src/server.rs and src/client.rs and src/main.rs to satisfy [[bin]] targets in Cargo.toml
# Cargo.toml defines 3 binaries: server (src/server.rs), client (src/client.rs), and implicit package bin (src/main.rs) probably?
# Wait, Cargo.toml has:
# [[bin]] name="server" path="src/server.rs"
# [[bin]] name="client" path="src/client.rs"
# And [package] name="balance" -> usually implies src/main.rs or src/lib.rs.
# The user has src/main.rs.

RUN mkdir -p src/operations
RUN echo "fn main() {}" > src/server.rs
RUN echo "fn main() {}" > src/client.rs
RUN echo "fn main() {}" > src/main.rs
# build.rs is needed for tonic-build
COPY build.rs ./
# and we need proto files providing they are referenced in build.rs
COPY proto ./proto

# Build dependencies
RUN cargo build --release --bin server

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Touch the main file to force rebuild of the application code
RUN touch src/server.rs

# Build the actual application
RUN cargo build --release --bin server

# Runtime stage
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /usr/src/balance/target/release/server ./server

EXPOSE 50051

ENTRYPOINT ["./server"]
