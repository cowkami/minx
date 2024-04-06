# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest as build

# Copy local code to the container image.
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=build /target/release/entry-server /entry-server

# Run the web service on container startup.
ENTRYPOINT ["/entry-server"]