# UDTool - Universal Data Tool

A command-line tool written in Rust for managing files on a remote server with API key authentication.

**Version:** 1.0.0  
**Author:** Ari Cummings  
**API Server:** https://UDTool.delphigamerz.xyz

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Usage](#usage)
5. [Commands Reference](#commands-reference)
6. [API Key Storage](#api-key-storage)
7. [Troubleshooting](#troubleshooting)

---

## Features

- **API Key Management**: Generate and validate API keys with secure storage
- **File Upload**: Upload files to the remote server with custom names
- **File Download**: Download files from the remote server to local disk
- **File Search**: Search for files by name on the remote server
- **File Deletion**: Delete files from the remote server
- **File Listing**: List all files stored on the remote server
- **Secure Authentication**: All operations require a valid API key sent via HTTP headers
- **Cross-platform Support**: Works on Windows, macOS, and Linux

---

## Installation

### Windows (MSI Installer - Recommended)

1. Download `UDTool-1.0.0.msi` from [Releases](https://github.com/ariplayz/UDTool/releases)
2. Double-click and follow the installer
3. Command available: `UDTool list`

### macOS (PKG Installer - Recommended)

1. Download `UDTool-1.0.0.pkg` from [Releases](https://github.com/ariplayz/UDTool/releases)
2. Double-click and follow the installer
3. Binary installed to: `/usr/local/bin/UDTool`
4. Open a new Terminal and run: `UDTool list`

### Linux/macOS (Binary)

1. Download `UDTool-linux-x86_64` or `UDTool-macos` from [Releases](https://github.com/ariplayz/UDTool/releases)
2. Make executable: `chmod +x UDTool-*`
3. Add to PATH: `sudo mv UDTool-* /usr/local/bin/UDTool`

### Build from Source

```bash
git clone https://github.com/ariplayz/UDTool.git
cd UDTool
cargo build --release
./target/release/UDTool list
```

### Build Installers

**macOS PKG:**
```bash
chmod +x create-macos-pkg.sh
./create-macos-pkg.sh
```

**Windows MSI:** (Requires WiX Toolset)
```powershell
cargo build --release
wix build wix\main.wxs -d Version=1.0.0 -d CargoTargetBinDir=target\release -ext WixToolset.UI.wixext -o target\wix\udtool.msi
```

---

## Configuration

### API Key Storage

UDTool stores your API key in a platform-specific directory:

- **Windows**: `%APPDATA%\UDTool\api_key.txt`
- **Linux**: `~/.config/UDTool/api_key.txt`
- **macOS**: `~/Library/Application Support/UDTool/api_key.txt`

The directory is created automatically on first use.

---

## Usage

### Quick Start

```bash
# Generate a new API key
UDTool genkey

# Upload a file
UDTool upload local_file.txt remote_file.txt

# Download a file
UDTool download remote_file.txt

# List all files
UDTool list

# Search for files
UDTool search "pattern"

# Delete a file
UDTool delete remote_file.txt
```

### API Key Commands

#### Generate New Key
```bash
UDTool genkey
```
Generates a new API key and saves it locally.

#### Check Existing Key
```bash
UDTool checkkey YOUR_API_KEY
```
Validates an API key and saves it if valid.

### File Operations

#### Upload
```bash
UDTool upload <local_path> <remote_name>
```

#### Download
```bash
UDTool download <remote_name>
```

#### List
```bash
UDTool list
```

#### Search
```bash
UDTool search <term>
```

#### Delete
```bash
UDTool delete <remote_name>
```

---

## Commands Reference

| Command | Syntax | Description |
|---------|--------|-------------|
| Generate Key | `UDTool genkey` | Generate and save new API key |
| Check Key | `UDTool checkkey <key>` | Validate and save API key |
| Upload | `UDTool upload <local> <remote>` | Upload a file |
| Download | `UDTool download <remote>` | Download a file |
| List | `UDTool list` | List all files |
| Search | `UDTool search <term>` | Search for files |
| Delete | `UDTool delete <remote>` | Delete a file |

---

## API Key Storage

### Security

- Keep your API key private (like a password)
- Don't share with untrusted parties
- Back up in a secure location
- On Unix systems, config directory permissions:
  ```bash
  chmod 700 ~/.config/UDTool
  chmod 600 ~/.config/UDTool/api_key.txt
  ```

### View Saved Key

**Windows:**
```cmd
type %APPDATA%\UDTool\api_key.txt
```

**Linux/macOS:**
```bash
cat ~/.config/UDTool/api_key.txt
```

---

## Troubleshooting

### "API key not found"
Generate a new key: `UDTool genkey`

### "Upload/Download failed with status: 401"
Invalid API key. Run: `UDTool checkkey YOUR_KEY`

### "File not found (404)"
File doesn't exist. List files: `UDTool list`

### Connection Issues
- Check internet connection
- Verify server is online: https://UDTool.delphigamerz.xyz
- Check firewall/proxy settings

---

## License

See `package.license` file.

---

## Support

Contact: ari@aricummings.com

---

**Version 1.0.0** | February 17, 2026

