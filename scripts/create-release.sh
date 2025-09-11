#!/bin/bash

# GalaxyOps Release Management Script
# Simplified version based on ripgrep release practices

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check project root
if [[ ! -f "Cargo.toml" ]]; then
    log_error "Must run from project root"
    exit 1
fi

# Get version from Cargo.toml
get_version() {
    grep -m 1 "version" Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/'
}

# Check if git is clean
check_git_clean() {
    if ! git diff-index --quiet HEAD --; then
        log_error "Git working directory is not clean"
        exit 1
    fi
}

# Create release
create_release() {
    local new_version=$1

    log_info "Creating release $new_version"

    # Update version files
    local current_version=$(get_version)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    else
        sed -i "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    fi

    echo "$new_version" > version.txt

    # Git operations
    git add Cargo.toml version.txt
    git commit -m "Bump version to $new_version"

    # Create annotated tag
    git tag -a "v$new_version" -m "Release version $new_version"

    log_success "Release $new_version created locally"
    log_info "Push with: git push origin main --tags"
}

# Main logic
main() {
    local version=${1:-""}

    if [[ -z "$version" ]]; then
        log_error "Usage: $0 <version> (e.g., 1.0.0)"
        exit 1
    fi

    check_git_clean
    create_release "$version"
}

main "$@"
