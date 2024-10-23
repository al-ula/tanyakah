# Start a new stage with a minimal image
FROM alpine

# Set the working directory in the container
WORKDIR /usr/local/bin

# Copy the binary from the builder stage
# COPY --from=builder /usr/src/tanyakah/target/release/tanyakah .

COPY target/x86_64-unknown-linux-musl/release/tanyakah /usr/local/bin

# Copy the necessary directories
COPY assets /usr/local/bin/assets
COPY components /usr/local/bin/components

RUN mkdir -p /root/iisa/tanyakah
RUN mkdir -p /usr/local/bin/db

# Expose the port your application listens on (change if necessary)
EXPOSE 8000

# Run the binary
CMD ["./tanyakah"]