# Release script for weather-exporter (Windows)
# Usage: .\scripts\release.ps1 [major|minor|patch]

param(
    [Parameter(Position=0)]
    [ValidateSet("major", "minor", "patch")]
    [string]$ReleaseType = "patch"
)

$ErrorActionPreference = "Stop"

# Check if working directory is clean
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Host "Error: Working directory is not clean. Commit or stash your changes first." -ForegroundColor Red
    exit 1
}

# Make sure we're on main branch
$currentBranch = git branch --show-current
if ($currentBranch -ne "main" -and $currentBranch -ne "master") {
    Write-Host "Warning: You're not on main/master branch. Current branch: $currentBranch" -ForegroundColor Yellow
    $response = Read-Host "Do you want to continue? (y/N)"
    if ($response -ne "y") {
        exit 1
    }
}

# Get current version from Cargo.toml
$cargoContent = Get-Content Cargo.toml
$versionLine = $cargoContent | Where-Object { $_ -match '^version = ".*"' } | Select-Object -First 1
$currentVersion = $versionLine -replace 'version = "(.+)"', '$1'

Write-Host "Current version: $currentVersion" -ForegroundColor Yellow

# Parse version components
$versionParts = $currentVersion.Split('.')
$major = [int]$versionParts[0]
$minor = [int]$versionParts[1]
$patch = [int]$versionParts[2]

# Calculate new version
switch ($ReleaseType) {
    "major" {
        $newVersion = "$($major + 1).0.0"
    }
    "minor" {
        $newVersion = "$major.$($minor + 1).0"
    }
    "patch" {
        $newVersion = "$major.$minor.$($patch + 1)"
    }
}

Write-Host "New version will be: $newVersion" -ForegroundColor Green
Write-Host ""
Write-Host "This will:"
Write-Host "  1. Update version in Cargo.toml"
Write-Host "  2. Update Cargo.lock"
Write-Host "  3. Commit the changes"
Write-Host "  4. Create tag v$newVersion"
Write-Host "  5. Push to origin"
Write-Host ""

$response = Read-Host "Do you want to proceed? (y/N)"
if ($response -ne "y") {
    Write-Host "Release cancelled"
    exit 1
}

# Update version in Cargo.toml
Write-Host "Updating Cargo.toml..." -ForegroundColor Green
$cargoContent = $cargoContent -replace '^version = ".*"', "version = `"$newVersion`""
Set-Content -Path Cargo.toml -Value $cargoContent

# Update Cargo.lock
Write-Host "Updating Cargo.lock..." -ForegroundColor Green
cargo update --workspace

# Run tests
Write-Host "Running tests..." -ForegroundColor Green
cargo test

# Run clippy
Write-Host "Running clippy..." -ForegroundColor Green
cargo clippy -- -D warnings

# Check formatting
Write-Host "Checking formatting..." -ForegroundColor Green
cargo fmt -- --check

# Commit changes
Write-Host "Committing changes..." -ForegroundColor Green
git add Cargo.toml Cargo.lock
git commit -m "chore: release v$newVersion"

# Create tag
Write-Host "Creating tag v$newVersion..." -ForegroundColor Green
git tag -a "v$newVersion" -m "Release v$newVersion"

# Push changes and tag
Write-Host "Pushing to origin..." -ForegroundColor Green
git push origin $currentBranch
git push origin "v$newVersion"

Write-Host ""
Write-Host "✅ Release v$newVersion created successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "GitHub Actions will now:"
Write-Host "  - Build binaries for all platforms"
Write-Host "  - Create Docker images"
Write-Host "  - Create a GitHub release with artifacts"
Write-Host ""
Write-Host "You can monitor the progress at:"
Write-Host "https://github.com/Joxtacy/weather-exporter/actions"
