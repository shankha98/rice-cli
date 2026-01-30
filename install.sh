#!/bin/bash
set -e

REPO="shankha98/rice-cli"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture
if [ "$ARCH" == "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" == "aarch64" ] || [ "$ARCH" == "arm64" ]; then
    if [ "$OS" == "darwin" ]; then
        echo "Note: Native arm64 build not available, using amd64 (Rosetta)."
        ARCH="amd64"
    else
        echo "Error: Linux arm64 not supported yet."
        exit 1
    fi
else
    echo "Error: Architecture $ARCH not supported."
    exit 1
fi

# Map OS to asset name
if [ "$OS" == "linux" ]; then
    ASSET="rice-cli-linux-amd64"
elif [ "$OS" == "darwin" ]; then
    ASSET="rice-cli-macos-amd64"
else
    echo "Error: OS $OS not supported."
    exit 1
fi

# Fetch latest release tag
LATEST_TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -z "$LATEST_TAG" ]; then
    echo "Error: Could not find latest release tag."
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$ASSET"

echo "Downloading $ASSET version $LATEST_TAG..."
curl -L -o rice-cli "$DOWNLOAD_URL"
chmod +x rice-cli

echo "Installing to /usr/local/bin (requires sudo)..."
if sudo mv rice-cli /usr/local/bin/rice; then
    echo "Successfully installed 'rice' to /usr/local/bin/rice"
    echo "Run 'rice setup' to get started."
else
    echo "Error: Failed to move binary to /usr/local/bin."
    exit 1
fi
