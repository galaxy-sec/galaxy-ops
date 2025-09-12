#!/bin/bash

# GalaxyOps Build Verification Script
# Comprehensive build testing across multiple targets
# Based on best practices from ripgrep and other major Rust projects

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

# Configuration
TARGETS=(
    "x86_64-unknown-linux-gnu"      # Standard Linux
    "x86_64-unknown-linux-musl"     # Static Linux
    "aarch64-unknown-linux-gnu"     # ARM64 Linux
    "x86_64-apple-darwin"          # macOS Intel
    "aarch64-apple-darwin"         # macOS Apple Silicon
)

# Current build target
CURRENT_TARGET=""
BUILD_SUCCESS=true

# Check if we're in the right directory
check_project_root() {
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Must run from project root directory containing Cargo.toml"
        exit 1
    fi
}

# Clean previous builds
clean_builds() {
    log_info "Cleaning previous build artifacts..."
    cargo clean
    rm -rf build-artifacts
    mkdir -p build-artifacts
}

# Install target if needed
install_target() {
    local target=$1
    log_info "Installing target: $target"
    rustup target add "$target" || {
        log_error "Failed to install target: $target"
        return 1
    }
    return 0
}

# Build for specific target
build_target() {
    local target=$1
    CURRENT_TARGET="$target"

    log_info "Building for target: $target"

    # Set build flags based on target
    local build_flags="--release --target $target"
    local env_vars=()

    case "$target" in
        *-musl)
            # Static linking for musl targets
            env_vars+=("RUSTFLAGS=-C target-feature=+crt-static -C link-arg=-static")
            env_vars+=("OPENSSL_STATIC=1")
            env_vars+=("PKG_CONFIG_ALL_STATIC=1")
            env_vars+=("LIBZ_SYS_STATIC=1")
            ;;
        *)
            # Standard dynamic linking
            env_vars+=("RUSTFLAGS=-C opt-level=3")
            ;;
    esac

    # Build with environment variables
    local env_prefix=""
    for env_var in "${env_vars[@]}"; do
        env_prefix+="$env_var "
    done

    if eval $env_prefix cargo build $build_flags; then
        log_success "Build successful for $target"

        # Verify the binary exists
        local binary_path="target/$target/release/gops"
        if [[ "$target" == *"windows"* ]]; then
            binary_path="${binary_path}.exe"
        fi

        if [[ -f "$binary_path" ]]; then
            log_success "Binary created: $binary_path"

            # Test binary
            test_binary "$binary_path" "$target"

            # Copy to artifacts
            local artifact_name="gops-$target"
            if [[ "$target" == *"windows"* ]]; then
                artifact_name="${artifact_name}.exe"
            fi
            cp "$binary_path" "build-artifacts/$artifact_name"

        else
            log_error "Binary not found: $binary_path"
            BUILD_SUCCESS=false
        fi
    else
        log_error "Build failed for $target"
        BUILD_SUCCESS=false
    fi
}

# Test binary functionality
test_binary() {
    local binary=$1
    local target=$2

    log_info "Testing binary for $target"

    # Test basic functionality
    if "$binary" --version >/dev/null 2>&1; then
        local version=$("$binary" --version 2>/dev/null | head -1)
        log_success "Version check passed: $version"
    else
        log_warning "Version check failed for $target"
    fi

    # Test help output
    if "$binary" --help >/dev/null 2>&1; then
        log_success "Help command works for $target"
    else
        log_warning "Help command failed for $target"
    fi

    # Static linking verification (for musl targets)
    if [[ "$target" == *"-mus
