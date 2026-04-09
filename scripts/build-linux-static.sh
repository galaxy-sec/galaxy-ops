#!/bin/bash
# Build script for static gops binary on Linux systems

set -e

echo "Building static gops binary..."

# Check if we're on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "This script should be run on Linux systems for optimal static compilation"
    echo "Current OS: $OSTYPE"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Add musl target if not already present
rustup target add x86_64-unknown-linux-musl

# Set environment variables for static compilation
export RUSTFLAGS="-C target-feature=+crt-static"
export OPENSSL_STATIC=1
export PKG_CONFIG_ALLOW_CROSS=1

# Build the static binary
echo "Compiling static binary..."
cargo build --release --target x86_64-unknown-linux-musl --bin gops

# Verify the build result
echo "Verifying static linking..."
BINARY="target/x86_64-unknown-linux-musl/release/gops"

if [[ -f "$BINARY" ]]; then
    echo "✓ Binary created successfully"
    file "$BINARY"

    # Check if it's statically linked
    if ldd "$BINARY" 2>&1 | grep -q "not a dynamic executable"; then
        echo "✓ Successfully created static binary"
        echo "Binary location: $BINARY"
        echo "Binary size: $(du -h "$BINARY" | cut -f1)"

        # Optional: strip the binary to reduce size
        read -p "Strip binary to reduce size? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            strip "$BINARY"
            echo "✓ Binary stripped"
            echo "New size: $(du -h "$BINARY" | cut -f1)"
        fi

        echo ""
        echo "Installation command:"
        echo "sudo cp $BINARY /usr/local/bin/gops"
        echo "sudo chmod +x /usr/local/bin/gops"

    else
        echo "⚠ Warning: Binary may not be fully static"
        echo "ldd output:"
        ldd "$BINARY"
    fi
else
    echo "✗ Build failed - binary not found"
    exit 1
fi

echo "Build complete!"
```

<USER>
状态：成功？
</USER>
