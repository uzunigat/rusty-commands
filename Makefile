# Define variables
IMAGE_NAME = rust_app_image
CONTAINER_NAME = rust_app_container

# Default target
all: build run

# Build the Docker image
build:
	@echo "Building Docker image..."
	docker-compose build

# Run the Docker container
run:
	@echo "Running Docker container..."
	docker-compose up --abort-on-container-exit

# Stop and remove the Docker container
stop:
	@echo "Stopping Docker container..."
	docker-compose down

# Clean up Docker images and containers
clean:
	@echo "Cleaning up Docker images and containers..."
	docker-compose down --rmi all --volumes --remove-orphans

# Clean and rebuild
rebuild: clean build

# Execute tests (assuming you have tests in your Rust project)
test:
	@echo "Running tests..."
	docker-compose run --rm rust_app cargo test

.PHONY: all build run stop clean rebuild test
