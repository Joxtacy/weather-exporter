# Release Process

This document describes the release process for the Weather Exporter project.

## Overview

The release process is semi-automated:
1. Version bump and tagging is done locally
2. GitHub Actions automatically builds and publishes releases

## Prerequisites

- You must have write access to the repository
- Your local main/master branch must be up to date
- Working directory must be clean (no uncommitted changes)
- Rust toolchain must be installed locally

## Release Types

We follow [Semantic Versioning](https://semver.org/):

- **Major** (x.0.0): Breaking changes
- **Minor** (0.x.0): New features, backwards compatible
- **Patch** (0.0.x): Bug fixes, backwards compatible

## Automated Release Process

### Using the release script (Recommended)

#### On macOS/Linux:
```bash
# Make the script executable (first time only)
chmod +x scripts/release.sh

# Create a patch release (bug fixes)
./scripts/release.sh patch

# Create a minor release (new features)
./scripts/release.sh minor

# Create a major release (breaking changes)
./scripts/release.sh major
```

#### On Windows:
```powershell
# Create a patch release (bug fixes)
.\scripts\release.ps1 patch

# Create a minor release (new features)
.\scripts\release.ps1 minor

# Create a major release (breaking changes)
.\scripts\release.ps1 major
```

The script will:
1. Check that your working directory is clean
2. Check that you're on main/master branch
3. Update the version in `Cargo.toml`
4. Update `Cargo.lock`
5. Run tests and linting
6. Commit the changes
7. Create an annotated Git tag
8. Push both the commit and tag to GitHub

## Manual Release Process

If you prefer to do it manually or the script doesn't work:

### 1. Update version in Cargo.toml

Edit `Cargo.toml` and update the version:
```toml
[package]
name = "weather-exporter"
version = "1.2.3"  # <- Update this
```

### 2. Update Cargo.lock

```bash
cargo update --workspace
```

### 3. Run tests and checks

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### 4. Commit the changes

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: release v1.2.3"
```

### 5. Create an annotated tag

```bash
git tag -a v1.2.3 -m "Release v1.2.3"
```

**Important**: The tag MUST start with `v` for the GitHub Actions to trigger.

### 6. Push changes and tag

```bash
git push origin main
git push origin v1.2.3
```

## After Pushing

Once you've pushed the tag, GitHub Actions will automatically:

1. Run tests on all platforms
2. Build binaries for:
   - Linux (amd64, arm64, armv7)
   - macOS (Intel, Apple Silicon)
   - Windows (amd64)
3. Build and push Docker images to:
   - Docker Hub: `joxtacy/weather-exporter`
   - GitHub Container Registry: `ghcr.io/joxtacy/weather-exporter`
4. Create a GitHub Release with:
   - All compiled binaries
   - Auto-generated changelog
   - Installation instructions

## Monitoring the Release

1. Go to [GitHub Actions](https://github.com/Joxtacy/weather-exporter/actions)
2. Watch the "Build and Test" workflow
3. Once complete, check the [Releases page](https://github.com/Joxtacy/weather-exporter/releases)

## Release Checklist

Before releasing, ensure:

- [ ] All tests pass locally
- [ ] Documentation is up to date
- [ ] CHANGELOG is updated (if maintaining one)
- [ ] Version number follows semantic versioning
- [ ] You're releasing from main/master branch
- [ ] No sensitive information in the code

## Troubleshooting

### Build fails after tagging

1. Check the [Actions tab](https://github.com/Joxtacy/weather-exporter/actions) for error details
2. Fix the issue locally
3. Delete the tag: `git push --delete origin v1.2.3` and `git tag -d v1.2.3`
4. Start the release process again

### Docker push fails

Ensure the following secrets are set in GitHub repository settings:
- `DOCKERHUB_USERNAME`: Your Docker Hub username
- `DOCKERHUB_TOKEN`: Docker Hub access token (not password)

### Release not created

- Ensure the tag starts with `v` (e.g., `v1.2.3`, not `1.2.3`)
- Check that the workflow file exists in `.github/workflows/build.yml`

## Version History Format

When creating releases, consider adding release notes describing:

- **New Features**: What's new in this release
- **Bug Fixes**: What issues were resolved
- **Breaking Changes**: Any backwards-incompatible changes
- **Deprecations**: Features that will be removed in future

Example:
```markdown
## What's Changed
### New Features
- Added support for multiple locations
- Added cache hit metrics

### Bug Fixes
- Fixed timestamp selection to use current weather data
- Improved error handling for API rate limits

### Breaking Changes
- Changed environment variable from WEATHER_LOCATION to WEATHER_LOCATIONS
```

## Rollback Process

If a release has critical issues:

1. **Don't delete the release** (for transparency)
2. Mark it as "Pre-release" in GitHub
3. Create a new patch release with the fix
4. Add a note in the broken release pointing to the fixed version

## Security Releases

For security fixes:
1. Don't disclose the vulnerability in commit messages
2. Release the fix as a patch version
3. After release, create a security advisory in GitHub
4. Consider yanking affected versions from crates.io (if published there)

## Questions?

If you have questions about the release process, please open an issue or discussion in the repository.
