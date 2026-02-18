#!/bin/bash

set -e

# Configuration
VERSION="1.0.0"
APP_NAME="UDTool"
PKG_FILE="${APP_NAME}-${VERSION}.pkg"
STAGING_DIR="pkg_staging"
INSTALL_PATH="/usr/local/bin"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}===============================================${NC}"
echo -e "${BLUE}    UDTool macOS PKG Installer Creator${NC}"
echo -e "${BLUE}===============================================${NC}"
echo ""

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${YELLOW}Warning: This script should be run on macOS${NC}"
    echo "Current OS: $OSTYPE"
fi

# Step 1: Check for Rust
echo -e "${BLUE}Step 1: Checking Rust installation...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi
echo -e "${GREEN}✓ Rust found${NC}"
echo ""

# Step 2: Build binaries
echo -e "${BLUE}Step 2: Building UDTool for macOS (Intel & Apple Silicon)...${NC}"
echo "  - Adding target architectures..."
rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true

echo "  - Building for x86_64 (Intel)..."
cargo build --release --target=x86_64-apple-darwin

echo "  - Building for aarch64 (Apple Silicon)..."
cargo build --release --target=aarch64-apple-darwin
echo -e "${GREEN}✓ Binaries built${NC}"
echo ""

# Step 3: Create universal binary
echo -e "${BLUE}Step 3: Creating universal binary...${NC}"
lipo -create \
  target/x86_64-apple-darwin/release/UDTool \
  target/aarch64-apple-darwin/release/UDTool \
  -output target/release/UDTool-universal

# Verify universal binary
ARCH_INFO=$(lipo -info target/release/UDTool-universal)
echo "  Architecture: $ARCH_INFO"
echo -e "${GREEN}✓ Universal binary created${NC}"
echo ""

# Step 4: Create staging directory
echo -e "${BLUE}Step 4: Creating package staging directory...${NC}"
rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR/usr/local/bin"

# Copy binary
cp target/release/UDTool-universal "$STAGING_DIR/usr/local/bin/UDTool"
chmod 755 "$STAGING_DIR/usr/local/bin/UDTool"

echo "  - Copied binary to $STAGING_DIR/usr/local/bin/"
echo -e "${GREEN}✓ Staging directory prepared${NC}"
echo ""

# Step 5: Create PKG installer
echo -e "${BLUE}Step 5: Creating PKG installer...${NC}"

# Create a temporary directory for PKG components
SCRIPTS_DIR=$(mktemp -d)
trap "rm -rf $SCRIPTS_DIR" EXIT

# Create postinstall script
cat > "$SCRIPTS_DIR/postinstall" << 'POSTINSTALL_EOF'
#!/bin/bash
# Add /usr/local/bin to PATH if not already there
echo "Installation completed successfully!"
echo "UDTool is now available at: /usr/local/bin/UDTool"
echo ""
echo "To use UDTool, you may need to:"
echo "1. Open a new Terminal window, or"
echo "2. Run: export PATH=\"/usr/local/bin:\$PATH\""
POSTINSTALL_EOF

chmod +x "$SCRIPTS_DIR/postinstall"

# Create the package
pkgbuild --root "$STAGING_DIR" \
  --scripts "$SCRIPTS_DIR" \
  --identifier "com.aricummings.udtool" \
  --version "$VERSION" \
  --install-location "/" \
  "$PKG_FILE"

echo -e "${GREEN}✓ PKG file created${NC}"
echo ""

# Step 6: Verify PKG
echo -e "${BLUE}Step 6: Verifying PKG...${NC}"
if pkgutil --check-signature "$PKG_FILE" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ PKG verification passed${NC}"
else
    echo -e "${YELLOW}⚠ PKG created (unsigned - optional to code sign)${NC}"
fi
echo ""

# Step 7: Cleanup
echo -e "${BLUE}Step 7: Cleaning up temporary files...${NC}"
rm -rf "$STAGING_DIR"
echo -e "${GREEN}✓ Cleanup complete${NC}"
echo ""

# Step 8: Generate checksum
echo -e "${BLUE}Step 8: Generating checksum...${NC}"
CHECKSUM=$(shasum -a 256 "$PKG_FILE" | cut -d' ' -f1)
echo "$CHECKSUM" > "${PKG_FILE}.sha256"
echo -e "${GREEN}✓ Checksum generated${NC}"
echo ""

# Step 9: Display results
PKG_SIZE=$(du -h "$PKG_FILE" | cut -f1)

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}✓ PKG Installer Created Successfully!${NC}"
echo -e "${GREEN}===============================================${NC}"
echo ""
echo "File Information:"
echo "  Name:     $PKG_FILE"
echo "  Size:     $PKG_SIZE"
echo "  SHA256:   $CHECKSUM"
echo ""
echo "Installation Instructions:"
echo "  1. double-click $PKG_FILE"
echo "  2. Follow the installer prompts"
echo "  3. Binary installed to: $INSTALL_PATH/UDTool"
echo ""
echo "After Installation:"
echo "  • Open a new Terminal window"
echo "  • Run: UDTool list"
echo ""
echo "Verification:"
echo "  shasum -a 256 $PKG_FILE"
echo "  Compare with: $CHECKSUM"
echo ""
echo "Distribution:"
echo "  Upload $PKG_FILE to GitHub Releases"
echo "  or your website"
echo ""

