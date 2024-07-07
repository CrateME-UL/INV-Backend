# Use an official Rust image as a parent image
FROM rust:1.79-slim-bullseye

# Set the working directory in the container
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    netcat \
    # postgresql-client \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy the current directory contents into the container at /app
COPY . .

# Make the entrypoint script executable
RUN chmod +x /app/scripts/entrypoint.sh

# Run the entrypoint script
ENTRYPOINT ["/app/scripts/entrypoint.sh"]
