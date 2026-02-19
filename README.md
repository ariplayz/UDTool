# UDTool - Universal Data Tool

A lightweight, cross-platform command-line utility for uploading, downloading, and managing files on a remote server using API key authentication.

**Version:** 1.3.0  
**Author:** Ari Cummings  
**API Server:** https://UDTool.delphigamerz.xyz

---

## Quick Links

- [ASP.NET API Repository](https://github.com/ariplayz/UDToolAPI)
- [UDTool Desktop Application](https://github.com/ariplayz/UDTool-Desktop)
- [GitHub Releases](https://github.com/ariplayz/UDTool/releases)

---

## Features

- ✅ **API Key Management**: Generate and validate API keys with secure local storage
- ✅ **File Operations**: Upload, download, delete, search, and list files
- ✅ **Progress Tracking**: Real-time progress bars for uploads and downloads
- ✅ **Secure Authentication**: All operations use API key authentication via HTTP headers
- ✅ **Cross-platform**: Windows, macOS (Intel & Apple Silicon), and Linux (x64 & ARM64)
- ✅ **Easy Installation**: Native installers for all platforms

---

## Installation

### Windows

Download and install the MSI for your architecture:
- **x64**: `UDTool-VERSION-x64.msi`
- **ARM64**: `UDTool-VERSION-arm64.msi`

Available from [Releases](https://github.com/ariplayz/UDTool/releases)

### macOS

Download and install the universal PKG installer:
- `UDTool-VERSION.pkg` (works on both Intel and Apple Silicon)

Available from [Releases](https://github.com/ariplayz/UDTool/releases)

Binary installed to: `/usr/local/bin/UDTool`

### Linux

**Debian/Ubuntu:**
```bash
sudo dpkg -i udtool_VERSION-1_amd64.deb      # x64
sudo dpkg -i udtool_VERSION-1_arm64.deb      # ARM64
```

**RHEL/CentOS/Fedora:**
```bash
sudo rpm -i udtool-VERSION-1.x86_64.rpm      # x64
sudo rpm -i udtool-VERSION-1.aarch64.rpm     # ARM64
```

Or use standalone binary:
```bash
chmod +x UDTool-linux-x64
sudo mv UDTool-linux-x64 /usr/local/bin/UDTool
```

Available from [Releases](https://github.com/ariplayz/UDTool/releases)

---

## Quick Start

### 1. Generate an API Key

```bash
UDTool genkey
```

Generates and saves a new API key locally.

### 2. Check Your API Key

```bash
UDTool checkkey              # Check stored key
UDTool checkkey YOUR_KEY_123 # Check specific key
```

### 3. Upload a File

```bash
UDTool upload /path/to/file.txt remote_name.txt
```

### 4. Download a File

```bash
UDTool download remote_name.txt
```

### 5. Search for Files

```bash
UDTool search filename_pattern
```

### 6. List All Files

```bash
UDTool list
```

### 7. Delete a File

```bash
UDTool delete filename.txt
```

---

## API Key Storage

Your API key is stored locally in your user's config directory:

| OS      | Location |
|---------|----------|
| Windows | `%APPDATA%\UDTool\api_key.txt` |
| macOS   | `~/.config/UDTool/api_key.txt` |
| Linux   | `~/.config/UDTool/api_key.txt` |

The stored key is used automatically for all authenticated operations.

---

## Server API Endpoints

The server provides these endpoints (all authenticated endpoints require `API-Key` header):

| Method | Endpoint | Requires Auth | Description |
|--------|----------|---------------|-------------|
| GET | `/key/check/{key}` | No | Validate an API key |
| POST | `/key/new` | No | Generate a new API key |
| POST | `/{fileName}` | Yes | Upload a file |
| GET | `/{fileName}` | Yes | Download a file |
| GET | `/list` | Yes | List all files |
| GET | `/search/{searchTerm}` | Yes | Search for files |
| DELETE | `/{fileName}` | Yes | Delete a file |

---

## Troubleshooting

### "API key not found"
Generate a new key: `UDTool genkey`

### "Key is not valid"
Verify your key: `UDTool checkkey <api_key>`

### Upload process killed
Possible causes:
- **Out of Memory**: Large files may exceed available RAM
- **Network timeout**: Check your network connection
- **File permissions**: Ensure file is readable

Try uploading a smaller file to test.

### No progress bar visible
Progress bars require ANSI support. Update to Windows Terminal or modern PowerShell on Windows.

---

## Building from Source

Requirements: Rust 1.70+

```bash
git clone https://github.com/ariplayz/UDTool.git
cd UDTool
cargo build --release
./target/release/UDTool
```

---

## Building Installers

### Windows MSI
Requires WiX Toolset v6.0+:
```powershell
wix build wix\main.wxs -d Version="1.0.0" -d CargoTargetBinDir=target\x86_64-pc-windows-msvc\release -d PlatformArch=x64
```

### macOS PKG
```bash
./create-macos-pkg.sh
```

### Linux DEB
Requires `cargo-deb`:
```bash
cargo install cargo-deb
cargo deb
```

### Linux RPM
Requires `fpm`:
```bash
fpm -s dir -t rpm -n udtool -v 1.0.0 -a x86_64 -C rpm_build -p udtool-1.0.0-1.x86_64.rpm usr/local/bin/UDTool
```

---

## Automated Releases

Push a version tag to trigger automatic builds:

```bash
git tag v1.3.0
git push origin main --tags
```

GitHub Actions automatically builds and publishes:
- Windows x64 & ARM64 (.exe + .msi)
- macOS universal (.pkg)
- Linux x64 & ARM64 (.deb + .rpm + executables)

---

## License

MIT License - See LICENSE file

## Support

For issues, feature requests, or questions:
- Open an issue on [GitHub](https://github.com/ariplayz/UDTool/issues)
- Contact: Ari Cummings <ari@aricummings.com>

