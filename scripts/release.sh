#!/bin/bash

# GalaxyFlow Release Script
# Based on ripgrep's release practices
# This script helps create and manage releases following the project's standards

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
check_project_root() {
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Must run from project root directory containing Cargo.toml"
        exit 1
    fi
}

# Get current version from Cargo.toml
get_cargo_version() {
    grep -m 1 "version" Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/'
}

# Check if git working directory is clean
check_git_clean() {
    if ! git diff-index --quiet HEAD --; then
        log_error "Git working directory is not clean. Please commit or stash changes."
        exit 1
    fi
}

# Validate version format (semver)
validate_version() {
    local version=$1
    if ! [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
        log_error "Invalid version format: $version. Must be semver (e.g., 1.0.0 or 1.0.0-alpha)"
        exit 1
    fi
}

# Check if version already exists
check_version_exists() {
    local version=$1
    if git tag | grep -q "^v$version$"; then
        log_error "Version v$version already exists as a git tag"
        exit 1
    fi
}

# Update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    local current_version=$(get_cargo_version)

    log_info "Updating version from $current_version to $new_version in Cargo.toml"

    # Update Cargo.toml
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    fi

    log_success "Updated Cargo.toml version to $new_version"
}

# Update version.txt
update_version_txt() {
    local new_version=$1
    echo "$new_version" > version.txt
    log_success "Updated version.txt to $new_version"
}

# Create and push git tag
create_git_tag() {
    local version=$1
    local tag_name="v$version"
    local message="Release $version"

    log_info "Creating git tag $tag_name"

    # Create annotated tag
    git tag -a "$tag_name" -m "$message"

    # Show tag info
    git show "$tag_name" --no-patch

    log_success "Created git tag $tag_name"
    log_info "Push this tag with: git push origin $tag_name"
}

# Local build test
build_local() {
    log_info "Performing local build test for x86_64-unknown-linux-gnu"

    # Clean build
    cargo clean
    cargo build --release --target x86_64-unknown-linux-gnu

    # Test binary
    ./target/x86_64-unknown-linux-gnu/release/gops --version

    log_success "Local build test passed"
}

# Static build test
build_static() {
    log_info "Performing static build test for x86_64-unknown-linux-musl"

    # Add musl target if not exists
    rustup target add x86_64-unknown-linux-musl

    # Clean and build
    cargo clean
    RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-static" \
    PKG_CONFIG_ALL_STATIC="1" \
    OPENSSL_STATIC="1" \
    cargo build --release --target x86_64-unknown-linux-musl

    # Verify static linking
    if file target/x86_64-unknown-linux-musl/release/gops | grep -q "static"; then
        log_success "Static build test passed"
    else
        log_warning
