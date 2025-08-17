#!/bin/bash
set -e

# Release script for weather-exporter
# Usage: ./scripts/release.sh [major|minor|patch]

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the release type (default to patch)
RELEASE_TYPE=${1:-patch}

# Validate release type
if [[ ! "$RELEASE_TYPE" =~ ^(major|minor|patch)$ ]]; then
    echo -e "${RED}Error: Invalid release type. Use major, minor, or patch${NC}"
    exit 1
fi

# Check if working directory is clean
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}Error: Working directory is not clean. Commit or stash your changes first.${NC}"
    exit 1
fi

# Make sure we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    echo -e "${YELLOW}Warning: You're not on main/master branch. Current branch: $CURRENT_BRANCH${NC}"
    read -p "Do you want to continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

# Calculate new version
case $RELEASE_TYPE in
    major)
        NEW_VERSION="$((MAJOR + 1)).0.0"
        ;;
    minor)
        NEW_VERSION="$MAJOR.$((MINOR + 1)).0"
        ;;
    patch)
        NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
        ;;
esac

echo -e "New version will be: ${GREEN}$NEW_VERSION${NC}"
echo
echo "This will:"
echo "  1. Update version in Cargo.toml"
echo "  2. Update Cargo.lock"
echo "  3. Commit the changes"
echo "  4. Create tag v$NEW_VERSION"
echo "  5. Push to origin"
echo
read -p "Do you want to proceed? (y/N) " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Release cancelled"
    exit 1
fi

# Update version in Cargo.toml
echo -e "${GREEN}Updating Cargo.toml...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Update Cargo.lock
echo -e "${GREEN}Updating Cargo.lock...${NC}"
cargo update --workspace

# Run tests to make sure everything still works
echo -e "${GREEN}Running tests...${NC}"
cargo test

# Run clippy
echo -e "${GREEN}Running clippy...${NC}"
cargo clippy -- -D warnings

# Check formatting
echo -e "${GREEN}Checking formatting...${NC}"
cargo fmt -- --check

# Commit changes
echo -e "${GREEN}Committing changes...${NC}"
git add Cargo.toml Cargo.lock
git commit -m "chore: release v$NEW_VERSION"

# Create tag
echo -e "${GREEN}Creating tag v$NEW_VERSION...${NC}"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

# Push changes and tag
echo -e "${GREEN}Pushing to origin...${NC}"
git push origin "$CURRENT_BRANCH"
git push origin "v$NEW_VERSION"

echo
echo -e "${GREEN}✅ Release v$NEW_VERSION created successfully!${NC}"
echo
echo "GitHub Actions will now:"
echo "  - Build binaries for all platforms"
echo "  - Create Docker images"
echo "  - Create a GitHub release with artifacts"
echo
echo "You can monitor the progress at:"
echo "https://github.com/Joxtacy/weather-exporter/actions"
