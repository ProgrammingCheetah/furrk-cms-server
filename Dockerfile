# ---- Builder Stage ----
# Use the official Rust image with all the build tools.
FROM rust:1.79-slim as builder

# Set the working directory.
WORKDIR /usr/src/app

# Copy over your manifests and create a dummy source file to cache dependencies.
# This layer will only be re-built if Cargo.toml or Cargo.lock changes.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# Now, copy your actual source code and build the real application.
COPY src ./src
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal "distroless" image which contains only our app and its dependencies.
# This makes the final image incredibly small and secure.
FROM gcr.io/distroless/cc-debian12

WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage.
# IMPORTANT: Replace `furrk-cms-server` with the actual name of your binary
# as defined in your Cargo.toml file.
COPY --from=builder /usr/src/app/target/release/furrk-cms-server .

# Set the command to run your application.
# The web server inside the container will run on port 8000.
CMD ["./furrk-cms-server"]
