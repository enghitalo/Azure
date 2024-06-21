# Use Alpine Linux as the base image
FROM alpine

# Set the working directory inside the container
WORKDIR /app

# Install system dependencies
RUN apk add --no-cache curl

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH"

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (without running the application)
RUN cargo build --release

# Copy the source code to the working directory
COPY src ./src

# Build the application
RUN cargo build --release

# Set the default command to run the application using `cargo run`
CMD ["cargo", "run", "--release"]