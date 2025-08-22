# Contributing to Weather Exporter

Thank you for your interest in contributing to Weather Exporter! This guide will help you get started with development and understand our release process.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Docker](https://docker.com/) (for testing Docker builds)

### Getting Started

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/weather-exporter.git
   cd weather-exporter
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Run the application**
   ```bash
   cargo run -- --user-agent 'dev/1.0 your-email@example.com' --locations Oslo
   ```

## Development Commands

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu
```

### Docker Development
```bash
# Build Docker image
docker build -t weather-exporter .

# Run with Docker
docker run -p 9090:9090 weather-exporter --user-agent 'test/1.0' --locations Oslo

# Test multi-arch build (requires buildx)
docker buildx build --platform linux/amd64,linux/arm64 .
```

## Code Guidelines

### Style
- Follow standard Rust formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add documentation for public APIs
- Keep functions focused and reasonably sized

### Testing
- Write unit tests for new functionality
- Add integration tests for complex features
- Test error conditions and edge cases
- Update tests when changing existing functionality

### Documentation
- Update README.md for user-facing changes
- Add inline documentation for complex code
- Update command-line help text if adding options

## Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write tests for new functionality
   - Ensure all tests pass
   - Run code formatting and linting

3. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add feature: description of what you added"
   ```

4. **Push to your fork and create a Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```

5. **PR Requirements**
   - All tests must pass
   - Code must be formatted (`cargo fmt`)
   - No clippy warnings
   - Include tests for new functionality
   - Update documentation if needed

## Release Process

### For Maintainers

Weather Exporter uses semantic versioning and automated releases via GitHub Actions.

#### Creating a Release

1. **Determine version bump**
   - **Patch** (1.0.x): Bug fixes, security updates
   - **Minor** (1.x.0): New features, backwards compatible
   - **Major** (x.0.0): Breaking changes

2. **Create and push a version tag**
   ```bash
   # For a new minor version (e.g., 1.2.0)
   git tag v1.2.0
   git push origin v1.2.0
   ```

3. **Automated process**
   The GitHub Actions workflow will automatically:
   - Run all tests and builds
   - Build binaries for all supported platforms:
     - Linux (amd64, arm64, armv7)
     - Windows (amd64)
     - macOS (amd64, arm64)
   - Build and push Docker images to:
     - Docker Hub: `joxtacy/weather-exporter`
     - GitHub Container Registry: `ghcr.io/joxtacy/weather-exporter`
   - Create a GitHub release with:
     - Auto-generated changelog
     - All binary artifacts
     - Installation instructions

#### Version Tag Examples

```bash
# Patch release (bug fixes)
git tag v1.0.1 && git push origin v1.0.1

# Minor release (new features)
git tag v1.1.0 && git push origin v1.1.0

# Major release (breaking changes)
git tag v2.0.0 && git push origin v2.0.0

# Pre-release versions
git tag v1.1.0-beta.1 && git push origin v1.1.0-beta.1
git tag v2.0.0-rc.1 && git push origin v2.0.0-rc.1
```

#### What Gets Built

**Binaries:**
- `weather-exporter-linux-amd64.tar.gz`
- `weather-exporter-linux-arm64.tar.gz`
- `weather-exporter-linux-armv7.tar.gz`
- `weather-exporter-windows-amd64.zip`
- `weather-exporter-macos-amd64.tar.gz`
- `weather-exporter-macos-arm64.tar.gz`

**Docker Images:**
- `joxtacy/weather-exporter:1.2.0`
- `joxtacy/weather-exporter:1.2`
- `joxtacy/weather-exporter:1`
- `joxtacy/weather-exporter:latest` (if main branch)
- `ghcr.io/joxtacy/weather-exporter:1.2.0`
- `ghcr.io/joxtacy/weather-exporter:1.2`
- `ghcr.io/joxtacy/weather-exporter:1`
- `ghcr.io/joxtacy/weather-exporter:latest` (if main branch)

#### Release Checklist

Before creating a release tag:

- [ ] All tests are passing on main branch
- [ ] Version number follows semantic versioning
- [ ] Update any version references in documentation
- [ ] Ensure CHANGELOG.md is up to date (if maintained)
- [ ] Docker Hub and GHCR credentials are configured in repository secrets

#### Troubleshooting Releases

**If release workflow fails:**
1. Check the Actions tab on GitHub for error details
2. Common issues:
   - Missing Docker Hub credentials (`DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN`)
   - GHCR permission issues (ensure `packages: write` permission)
   - Test failures on specific platforms
   - Cross-compilation toolchain issues

**If Docker push fails:**
1. Verify repository secrets are set correctly
2. Check that GHCR package visibility allows pushes
3. Ensure the package exists or can be created

## Issue Reporting

When reporting issues, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Weather Exporter version
- Command used and full output
- Expected vs actual behavior

## Getting Help

- Check existing [Issues](https://github.com/Joxtacy/weather-exporter/issues)
- Create a new issue for bugs or feature requests
- Discussions for questions and ideas

## Code of Conduct

Be respectful and constructive in all interactions. We want this to be a welcoming space for contributors of all backgrounds and experience levels.