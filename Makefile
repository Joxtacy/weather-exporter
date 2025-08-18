.PHONY: help build test clean run fmt lint release-patch release-minor release-major docker docker-run check

# Default target
help:
	@echo "Weather Exporter - Development Commands"
	@echo ""
	@echo "Development:"
	@echo "  make build         - Build the project in debug mode"
	@echo "  make release       - Build the project in release mode"
	@echo "  make run           - Run with default location (Oslo)"
	@echo "  make test          - Run all tests"
	@echo "  make check         - Run all checks (fmt, clippy, test)"
	@echo "  make fmt           - Format code"
	@echo "  make lint          - Run clippy linter"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  make docker        - Build Docker image"
	@echo "  make docker-run    - Run Docker container"
	@echo "  make docker-push   - Push to Docker Hub (requires login)"
	@echo ""
	@echo "Release:"
	@echo "  make release-patch - Create a patch release (0.0.x)"
	@echo "  make release-minor - Create a minor release (0.x.0)"
	@echo "  make release-major - Create a major release (x.0.0)"
	@echo ""
	@echo "Examples:"
	@echo "  make run USER_AGENT='my-app/1.0' LOCATIONS='Stockholm,Oslo'"
	@echo "  make docker-run WEATHER_USER_AGENT='my-app/1.0 github.com/user/repo'"

# Build commands
build:
	cargo build

release:
	cargo build --release

# Run the application
LOCATIONS ?= Oslo
USER_AGENT ?= weather-exporter-dev/1.0 local-development
run:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)"

run-debug:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)" --log-level debug

# Testing and checking
test:
	cargo test

check: fmt lint test
	@echo "✅ All checks passed!"

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Docker commands
DOCKER_IMAGE ?= weather-exporter
DOCKER_TAG ?= latest

docker:
	docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) .

docker-run:
	@if [ -z "$WEATHER_USER_AGENT" ]; then \
		echo "Error: WEATHER_USER_AGENT environment variable is required"; \
		echo "Example: export WEATHER_USER_AGENT='my-app/1.0 github.com/user/repo'"; \
		exit 1; \
	fi
	docker run --rm -p 9090:9090 \
		-e WEATHER_USER_AGENT="$WEATHER_USER_AGENT" \
		-e WEATHER_LOCATIONS="$(LOCATIONS)" \
		$(DOCKER_IMAGE):$(DOCKER_TAG)

docker-push:
	docker push $(DOCKER_IMAGE):$(DOCKER_TAG)

# Multi-arch Docker build (requires buildx)
docker-buildx:
	docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t $(DOCKER_IMAGE):$(DOCKER_TAG) .

# Release commands
release-patch:
	@bash scripts/release.sh patch

release-minor:
	@bash scripts/release.sh minor

release-major:
	@bash scripts/release.sh major

# Development setup
setup:
	rustup component add rustfmt clippy
	@echo "✅ Development environment ready!"

# Watch for changes and rebuild
watch:
	cargo watch -x build -x test

# Run with specific log level
run-info:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)" --log-level info

run-warn:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)" --log-level warn

run-error:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)" --log-level error

# Check configuration
check-config:
	cargo run -- --user-agent "$(USER_AGENT)" --locations "$(LOCATIONS)" --check

# Metrics check - curl the metrics endpoint
metrics:
	@echo "Fetching metrics from http://localhost:9090/metrics"
	@curl -s http://localhost:9090/metrics | head -20
	@echo "..."
	@echo "(showing first 20 lines)"

# Security audit
audit:
	cargo audit

# Update dependencies
update:
	cargo update

# Generate documentation
docs:
	cargo doc --open

# Install the binary locally
install:
	cargo install --path .

# Uninstall the binary
uninstall:
	cargo uninstall weather-exporter
