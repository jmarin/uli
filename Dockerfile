# Build environment
FROM ekidd/rust-musl-builder as builder

# Copy source code
ADD . ./

# Fix permissions on source code
RUN sudo chown -R rust:rust /home/rust

# Build application
RUN cargo build --release

# Create Docker image, copying output from builder image
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/uli \
  /usr/local/bin/
CMD /usr/local/bin/uli

