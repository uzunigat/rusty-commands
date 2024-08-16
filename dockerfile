# Use the official Rust image from Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm -f src/main.rs

# Copy the rest of the source code to the container
COPY . .

# Build the application
RUN cargo build --release

# Run the application
CMD ["cargo", "run"]
