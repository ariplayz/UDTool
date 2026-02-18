# UDTool - Universal Data Tool

A command-line tool written in Rust for managing files on a remote server with API key authentication.

**Version:** 1.0.0  
**Author:** Ari Cummings  
**API Server:** https://UDTool.delphigamerz.xyz

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
   - [Windows MSI Installer](#option-1-windows-msi-installer-recommended-for-windows-users)
   - [Linux/macOS Executable](#option-2-linuxmacos-executable-pre-compiled-binary)
   - [macOS Installer (Coming Soon)](#option-3-macos-installer-coming-soon)
   - [Build from Source](#option-4-build-from-source)
   - [Build Custom Installer](#option-5-build-windows-installer-from-source)
3. [Configuration](#configuration)
4. [Usage](#usage)
   - [API Key Management](#api-key-management)
   - [File Operations](#file-operations)
5. [Commands Reference](#commands-reference)
6. [API Key Storage](#api-key-storage)
7. [Error Handling](#error-handling)
8. [Architecture](#architecture)
9. [WiX Installer Configuration](#wix-installer-configuration)
10. [Building and Development](#building-and-development)
11. [Troubleshooting](#troubleshooting)

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
- **Configurable Storage**: API keys stored in OS-specific config directories

---

## Installation

### Option 1: Windows MSI Installer (Recommended for Windows Users)

The easiest way to install UDTool on Windows is using the prebuilt MSI installer.

#### Prerequisites

- Windows 7 or later
- Administrator privileges (for installation)

#### Installation Steps

1. **Download the MSI Installer**
   - Download `UDTool-1.0.0.msi` from the [Releases page](https://github.com/ariplayz/UDTool/releases)

2. **Run the Installer**
   - Double-click the `.msi` file
   - Follow the installation wizard
   - Choose installation location (default: `C:\Program Files\UDTool\`)
   - Click "Install"

3. **Verify Installation**
   ```powershell
   UDTool --help
   ```
   Or open a new PowerShell/Command Prompt and run:
   ```powershell
   UDTool list
   ```

4. **Uninstall**
   - Control Panel > Programs > Programs and Features
   - Find "UDTool"
   - Click "Uninstall"

#### What the MSI Installer Does

- Creates application directory at `C:\Program Files\UDTool\`
- Installs the `UDTool.exe` binary
- Adds UDTool to system PATH (accessible from any command line)
- Creates Start Menu shortcuts
- Registers with Windows Add/Remove Programs
- Easy uninstallation via Control Panel

### Option 2: Linux/macOS Executable (Pre-compiled Binary)

For Linux and macOS users, pre-compiled binaries are available for direct download without building.

#### Prerequisites

- Linux (x86_64) or macOS (Intel/Apple Silicon)
- No additional software required

#### Installation Steps

1. **Download the Executable**
   - Visit the [Releases page](https://github.com/ariplayz/UDTool/releases)
   - Download `UDTool-linux-x86_64` for Linux
   - Download `UDTool-macos` for macOS (works on both Intel and Apple Silicon)

2. **Make Executable**
   ```bash
   chmod +x UDTool-linux-x86_64
   # or
   chmod +x UDTool-macos
   ```

3. **Add to PATH (Optional)**
   ```bash
   sudo mv UDTool-linux-x86_64 /usr/local/bin/UDTool
   # or
   sudo mv UDTool-macos /usr/local/bin/UDTool
   ```
   
   Then you can run `UDTool` from anywhere.

4. **Run the Tool**
   ```bash
   # If you added to PATH
   UDTool list
   
   # Or if running from current directory
   ./UDTool-linux-x86_64 list
   # or
   ./UDTool-macos list
   ```

#### What's Included

- Single executable binary
- No dependencies to install
- Ready to use immediately
- Cross-compatible across Linux distributions
- Universal binary for macOS (Intel and Apple Silicon)

### Option 3: macOS Installer (Coming Soon)

A native macOS `.dmg` installer is coming soon for even easier installation on macOS systems.

#### Expected Features

- **Drag-and-Drop Installation**: Traditional macOS installer experience
- **Automatic PATH Setup**: UDTool automatically added to system PATH
- **Uninstall Support**: Easy removal via Applications folder
- **Code Signed**: Signed and notarized for security
- **System Integration**: Full macOS integration with Launch Services

#### When Available

- Check the [Releases page](https://github.com/ariplayz/UDTool/releases) for updates
- Subscribe to release notifications for announcement

#### What to Do Now (Before macOS Installer)

Until the macOS installer is available, use the pre-compiled binary:

```bash
# Download UDTool-macos from Releases
chmod +x UDTool-macos
sudo mv UDTool-macos /usr/local/bin/UDTool
UDTool list
```

### Option 4: Build from Source

For development or custom builds on any platform:

#### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

#### Build Steps

```bash
git clone https://github.com/ariplayz/UDTool.git
cd UDTool
cargo build --release
```

The compiled binary will be available at:
- **Windows**: `target/release/UDTool.exe`
- **Linux/macOS**: `target/release/UDTool`

#### Add to PATH

To use `UDTool` from anywhere after building:

**Windows:**
```powershell
# Add the release directory to your system PATH
# Control Panel > System > Environment Variables > Edit environment variables
# Add: C:\Users\YourUsername\RustroverProjects\UDTool\target\release
```

**Linux/macOS:**
```bash
sudo cp target/release/UDTool /usr/local/bin/
```

### Option 5: Build Windows Installer from Source

If you need to create a custom Windows installer:

#### Prerequisites

- [WiX Toolset 4.0+](https://wixtoolset.org/releases/)
- Windows platform
- Rust and Cargo installed
- Visual Studio or Visual Studio Build Tools (for WiX)

#### Build Steps

```powershell
# Step 1: Build the release binary
cargo build --release

# Step 2: Run the WiX build command
wix build wix\main.wxs `
  -d Version=1.0.0 `
  -d CargoTargetBinDir=target\release `
  -ext WixToolset.UI.wixext `
  -o target\wix\udtool.msi

# Step 3: The installer will be created at target/wix/udtool.msi
```

The resulting `udtool.msi` file can be distributed to users.

---

## Configuration

### API Key Storage Location

UDTool automatically stores your API key in a platform-specific configuration directory:

- **Windows**: `%APPDATA%\UDTool\api_key.txt`  
  (e.g., `C:\Users\YourUsername\AppData\Roaming\UDTool\api_key.txt`)
- **Linux**: `~/.config/UDTool/api_key.txt`
- **macOS**: `~/Library/Application Support/UDTool/api_key.txt`

The directory will be created automatically when you first save an API key.

### Manual API Key Setup

If you prefer to set the API key manually:

1. Generate or obtain a valid API key from the server
2. Create the configuration directory for your platform
3. Create a file named `api_key.txt` in that directory
4. Paste your API key into the file and save

---

## Usage

### Quick Start

1. **Generate a new API key** (first-time setup):
   ```bash
   UDTool genkey
   ```

2. **Verify your API key** (if you already have one):
   ```bash
   UDTool checkkey YOUR_API_KEY_HERE
   ```

3. **Upload a file**:
   ```bash
   UDTool upload local_file.txt remote_file.txt
   ```

4. **Download a file**:
   ```bash
   UDTool download remote_file.txt
   ```

5. **List all files**:
   ```bash
   UDTool list
   ```

### API Key Management

#### Generate a New API Key

```bash
UDTool genkey
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Generating new API key...
New API key generated: 550e8400-e29b-41d4-a716-446655440000
API key saved successfully.
```

The generated API key is automatically saved to your configuration directory.

#### Validate and Import an Existing API Key

If you already have an API key from another source:

```bash
UDTool checkkey YOUR_API_KEY_HERE
```

**Example:**
```bash
UDTool checkkey 550e8400-e29b-41d4-a716-446655440000
```

**Output (Valid Key):**
```
UDTool v1.0 by Ari Cummings

Checking API key...
Key is valid.
API key saved successfully.
```

**Output (Invalid Key):**
```
UDTool v1.0 by Ari Cummings

Checking API key...
Key is not valid.
```

---

### File Operations

#### Upload a File

Upload a file from your local system to the remote server.

```bash
UDTool upload <local_file_path> <remote_file_name>
```

**Example:**
```bash
UDTool upload C:\Documents\report.pdf my_report.pdf
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Uploading C:\Documents\report.pdf...
File successfully uploaded.
File URL: https://UDTool.delphigamerz.xyz/my_report.pdf
```

**Notes:**
- The remote file name can be different from the local file name
- Files are stored in your user-specific directory on the server
- Maximum timeout: 2000 seconds

#### Download a File

Download a file from the remote server to your local system.

```bash
UDTool download <remote_file_name>
```

**Example:**
```bash
UDTool download my_report.pdf
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Downloading my_report.pdf...
Downloaded my_report.pdf...
```

**Notes:**
- Files are saved to the current working directory
- If a file with the same name exists, it will be overwritten

#### List All Files

List all files stored on the remote server under your account.

```bash
UDTool list
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Listing all files...
Files:
  - my_report.pdf
  - document.docx
  - image.png
  - archive.zip
```

**Notes:**
- Only files in your user-specific directory are listed
- File count is displayed
- Files are sorted alphabetically

#### Search for Files

Search for files by name pattern on the remote server.

```bash
UDTool search <search_term>
```

**Example:**
```bash
UDTool search report
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Searching for report...
Found 2 file(s):
  - my_report.pdf
  - annual_report.xlsx
```

**Notes:**
- Search is case-sensitive
- Searches for files containing the search term in their name
- Uses wildcard matching (`*search_term*`)

#### Delete a File

Delete a file from the remote server.

```bash
UDTool delete <remote_file_name>
```

**Example:**
```bash
UDTool delete old_file.txt
```

**Output:**
```
UDTool v1.0 by Ari Cummings

Deleting old_file.txt...
File 'old_file.txt' successfully deleted.
```

**Notes:**
- This action is permanent and cannot be undone
- You can only delete files in your user-specific directory

---

## Commands Reference

| Command | Syntax | Description |
|---------|--------|-------------|
| Generate Key | `UDTool genkey` | Generate a new API key and save it locally |
| Check Key | `UDTool checkkey <key>` | Validate an API key and save it if valid |
| Upload | `UDTool upload <local_path> <remote_name>` | Upload a file to the server |
| Download | `UDTool download <remote_name>` | Download a file from the server |
| List | `UDTool list` | List all files on the server |
| Search | `UDTool search <term>` | Search for files by name |
| Delete | `UDTool delete <remote_name>` | Delete a file from the server |

---

## API Key Storage

### Storage Details

- **Location**: OS-specific configuration directory (automatically managed)
- **Format**: Plain text file containing the API key
- **Permissions**: File is created with restricted permissions (readable by owner only on Unix systems)
- **Automatic Creation**: Directory and file are created automatically on first use

### Security Considerations

1. **Keep Your Key Safe**: Your API key grants access to your files on the server. Treat it like a password.
2. **Don't Share**: Never share your API key with untrusted parties.
3. **File Permissions**: On Unix systems, ensure the configuration directory has restricted permissions:
   ```bash
   chmod 700 ~/.config/UDTool
   chmod 600 ~/.config/UDTool/api_key.txt
   ```
4. **Backup**: Consider backing up your API key in a secure location if you need to reinstall the tool.

### Retrieving Your API Key

To view your currently saved API key:

**Windows:**
```bash
type %APPDATA%\UDTool\api_key.txt
```

**Linux/macOS:**
```bash
cat ~/.config/UDTool/api_key.txt
```

---

## Error Handling

### Common Errors and Solutions

#### "API key not found. Use 'genkey' command to generate a new key."

**Cause:** You haven't generated or imported an API key yet.

**Solution:**
```bash
UDTool genkey
# or
UDTool checkkey YOUR_EXISTING_KEY
```

#### "Check failed with status: 401"

**Cause:** Invalid API key.

**Solution:**
```bash
UDTool checkkey YOUR_CORRECT_KEY
```

#### "Upload failed with status: 400"

**Cause:** Missing or invalid file path.

**Solution:**
```bash
# Verify the file exists and use the correct path
UDTool upload "C:\path\to\existing\file.txt" remote_name.txt
```

#### "Download failed with status: 404"

**Cause:** File doesn't exist on the server.

**Solution:**
```bash
# List files to see what's available
UDTool list
# Then download an existing file
UDTool download existing_file.txt
```

#### "Connection timeout"

**Cause:** Server is unreachable or network is unstable.

**Solution:**
- Check your internet connection
- Verify the server is online at https://UDTool.delphigamerz.xyz
- Wait a moment and try again
- Check your firewall/proxy settings

### Server Response Messages

The server provides detailed error messages in response to API calls:

```
Upload failed with status: 401
Server response: Key is not valid.
```

Read the server response carefully to diagnose the issue.

---

## Architecture

### Project Structure

```
UDTool/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project manifest and dependencies
├── Cargo.lock           # Locked dependency versions
├── Cross.toml           # Cross-compilation configuration
├── wix/                 # Windows installer source
├── README.md            # This file
└── target/              # Build output directory
    └── release/
        └── UDTool.exe   # Compiled binary (Windows)
```

### Code Organization

The `main.rs` file contains:

#### API Key Management Functions

- **`get_api_key_file()`** - Determines the platform-specific storage location
- **`load_api_key()`** - Loads the API key from disk, returns error if not found
- **`save_api_key(key)`** - Saves the API key to the platform-specific config directory
- **`generate_key()`** - Calls the `/key/new` endpoint to generate a new API key from the server
- **`check_key(args)`** - Calls the `/key/check/{key}` endpoint to validate an existing API key

#### File Operation Functions

- **`upload(args, client, base_url, api_key)`** - Uploads a file using multipart form data (POST)
- **`download(args, client, base_url, api_key)`** - Downloads a file and saves it locally (GET)
- **`list_files(client, base_url, api_key)`** - Lists all files in your server directory (GET)
- **`search(args, client, base_url, api_key)`** - Searches for files by name pattern (GET)
- **`delete(args, client, base_url, api_key)`** - Deletes a file from the server (DELETE)

#### Utility Functions

- **`main()`** - Entry point, handles command parsing and routing via match statement
- **`print_usage()`** - Displays help information

### Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `reqwest` | 0.12 | HTTP client with blocking, JSON, and multipart support |
| `dirs` | 6.0.0 | Cross-platform config directory detection |
| `serde_json` | 1.0 | JSON parsing and serialization |
| `std` | Built-in | File I/O and environment variables |

### HTTP Headers

All file operation requests include:

```
API-Key: <your_api_key>
```

This header is required by the ASP.NET server for authentication.

### Network Configuration

- **Base URL**: `https://UDTool.delphigamerz.xyz`
- **Protocol**: HTTPS (secure)
- **Timeout**: 2000 seconds (for large file transfers)
- **User-Agent**: reqwest default

### Request/Response Flow

1. **File Operation Command**
   ```
   UDTool upload file.txt remote.txt
   ```

2. **API Key Loading**
   ```
   Reads: %APPDATA%\UDTool\api_key.txt
   ```

3. **HTTP Request**
   ```
   POST https://UDTool.delphigamerz.xyz/remote.txt
   Header: API-Key: YOUR_KEY
   Body: Multipart form data (file)
   ```

4. **Server Processing**
   - Validates API key against stored keys
   - Stores file in user-specific directory
   - Returns JSON response with status

5. **Result Display**
   ```
   File successfully uploaded.
   File URL: https://UDTool.delphigamerz.xyz/remote.txt
   ```

---

## Building and Development

### Build Commands

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized, recommended for distribution)
cargo build --release

# Check for errors without building
cargo check

# Run directly
cargo run -- <command> <args>
```

### Example Development Workflow

```bash
# Make changes to src/main.rs
nano src/main.rs

# Check for errors
cargo check

# Build and test
cargo build --release

# Test the binary
./target/release/UDTool list
```

### Creating Windows Installer

The project includes WiX configuration for creating an MSI installer. Build the Windows installer with:

```powershell
# Build release binary first
cargo build --release

# Then run the WiX build command
wix build wix\main.wxs `
  -d Version=1.0.0 `
  -d CargoTargetBinDir=target\release `
  -ext WixToolset.UI.wixext `
  -o target\wix\udtool.msi
```

**Requirements:**
- [WiX Toolset](https://wixtoolset.org/) must be installed
- Windows platform for building

---

## WiX Installer Configuration

### Overview

UDTool uses WiX (Windows Installer XML) to create professional Windows MSI installers. The configuration is defined in `wix/main.wxs`.

### WiX File Structure

```
wix/
└── main.wxs          # WiX installer configuration
```

### Installer Features

The WiX installer provides:

- **Installation to Program Files**: Installs to `C:\Program Files\UDTool\` by default
- **System PATH Integration**: Automatically adds UDTool to system PATH for command-line access
- **Start Menu Shortcuts**: Creates shortcuts for easy access
- **Uninstall Support**: Full uninstallation via Windows Add/Remove Programs
- **User-Friendly Interface**: Professional installation wizard
- **Version Management**: Automatically handles version upgrades

### Building the MSI

#### Command Breakdown

```powershell
wix build wix\main.wxs `
  -d Version=1.0.0 `
  -d CargoTargetBinDir=target\release `
  -ext WixToolset.UI.wixext `
  -o target\wix\udtool.msi
```

| Parameter | Description |
|-----------|-------------|
| `wix build` | WiX command to build the installer |
| `wix\main.wxs` | Path to the WiX source file |
| `-d Version=1.0.0` | Define version variable |
| `-d CargoTargetBinDir=target\release` | Path to compiled binary |
| `-ext WixToolset.UI.wixext` | Enable UI extension for installer UI |
| `-o target\wix\udtool.msi` | Output file path |

### Customizing the Installer

#### Changing Version Number

To build with a different version:

```powershell
wix build wix\main.wxs `
  -d Version=1.0.1 `
  -d CargoTargetBinDir=target\release `
  -ext WixToolset.UI.wixext `
  -o target\wix\udtool.msi
```

#### Changing Installation Directory

To install to a different location, edit `wix/main.wxs` and modify the installation path variable.

#### Building Silent Installation

For automated deployments:

```powershell
# Run MSI with silent flag
msiexec /i target\wix\udtool.msi /quiet /norestart
```

Or with logging:

```powershell
msiexec /i target\wix\udtool.msi /quiet /norestart /l*v build.log
```

### Installer Distribution

#### Preparing for Release

1. **Build the MSI**
   ```powershell
   cargo build --release
   wix build wix\main.wxs `
     -d Version=1.0.0 `
     -d CargoTargetBinDir=target\release `
     -ext WixToolset.UI.wixext `
     -o target\wix\udtool.msi
   ```

2. **Verify the MSI**
   - Test the installer on a clean Windows machine
   - Verify command-line access works
   - Check Start Menu shortcuts
   - Test uninstallation

3. **Sign the MSI (Optional but Recommended)**
   ```powershell
   signtool sign /f certificate.pfx /p password `
     /t http://timestamp.server.com `
     target\wix\udtool.msi
   ```

4. **Upload for Distribution**
   - Create a GitHub Release
   - Upload `udtool.msi` file
   - Provide download link in documentation

#### Installation by End Users

Users can install in several ways:

**Method 1: Double-click the MSI**
- Download `udtool.msi`
- Double-click to launch installer
- Follow the wizard

**Method 2: Command Line**
```powershell
msiexec /i udtool.msi
```

**Method 3: PowerShell**
```powershell
Start-Process msiexec.exe -ArgumentList "/i `"C:\path\to\udtool.msi`"" -Wait
```

### Troubleshooting WiX Build

#### Error: "WiX Toolset not installed"

**Solution:**
```powershell
# Download from https://wixtoolset.org/releases/
# Install WiX Toolset 4.0 or later
# Add WiX to PATH or use full path
"C:\Program Files (x86)\WiX Toolset v4.0\bin\wix.exe" build wix\main.wxs
```

#### Error: "Cannot find compiled binary"

**Solution:**
```powershell
# Make sure to build release first
cargo build --release

# Check that target\release\UDTool.exe exists
dir target\release\UDTool.exe

# Verify the correct path in the build command
wix build wix\main.wxs `
  -d Version=1.0.0 `
  -d CargoTargetBinDir=target\release `  # This must match your binary location
  -ext WixToolset.UI.wixext `
  -o target\wix\udtool.msi
```

#### Error: "UI extension not found"

**Solution:**
```powershell
# Ensure WiX UI extension is installed
# Update WiX Toolset
dotnet tool update --global wix

# Or install fresh
dotnet tool install --global wix --version 4.0.0
```

### Installer Upgrade and Version Management

#### Upgrading from Previous Version

The WiX installer supports automatic upgrades:

1. **Users with previous version** run the new MSI
2. **WiX detects** the previous version and offers upgrade
3. **Settings and data** are preserved
4. **New version** is installed alongside old

#### Version Number Management

Keep version numbers in sync:

- **Cargo.toml**: `version = "1.0.0"`
- **WiX build command**: `-d Version=1.0.0`
- **GitHub Releases**: Tag as `v1.0.0`

---

## Advanced Installer Topics

### Installer Requirements Check

The WiX installer checks for:

- Windows 7 or later
- Administrator privileges for installation
- Sufficient disk space

### Registry Integration

The installer registers UDTool in Windows registry for:

- Add/Remove Programs list
- Uninstallation support
- Version tracking

### PATH Environment Variable

The installer automatically:

- Adds `C:\Program Files\UDTool\` to system PATH
- Makes UDTool accessible from any command prompt
- Restores original PATH on uninstall

### Start Menu Shortcuts

The installer creates shortcuts for:

- Main application (appears in Start Menu)
- Uninstall program
- Quick access to application directory

---

## Troubleshooting

### API Key Issues

**Q: I lost my API key. Can I recover it?**  
A: No, API keys cannot be recovered. You'll need to generate a new one with `UDTool genkey`.

**Q: Can I use the same API key on multiple machines?**  
A: Yes, use `UDTool checkkey <key>` to import the same key on different machines.

**Q: How do I change my API key?**  
A: Simply run `UDTool genkey` to create a new key, or use `UDTool checkkey <new_key>` to import a different one.

**Q: Where is my API key stored?**  
A: It's stored in your OS-specific configuration directory. Run the tool once and check the appropriate location listed in the Configuration section.

### File Operation Issues

**Q: Can I upload files with special characters in the name?**  
A: Yes, but special characters will be preserved. Avoid characters that are invalid in file paths.

**Q: What's the maximum file size?**  
A: Depends on server configuration. Contact the server administrator for limits.

**Q: Can I resume interrupted uploads?**  
A: No, uploads restart from the beginning if interrupted. The connection timeout is set to 2000 seconds.

### Compilation Issues

**Q: I get "error: could not compile" when building**  
A: Ensure you have Rust 1.70+ installed. Run `rustup update` to update your Rust installation.

**Q: The build is very slow**  
A: This is normal for the first build. Subsequent builds will be faster. Use `cargo build --release` for optimized output.

---

## Performance Tips

1. **Large Files**: For files over 100MB, consider uploading during off-peak hours
2. **Multiple Operations**: Run multiple UDTool instances in parallel if needed
3. **Network Speed**: The 2000-second timeout should handle most network conditions
4. **Local Storage**: Keep the config directory on fast local storage, not network drives

---

## License

See `package.license` file in the repository.

---

## Support and Contributions

For issues, feature requests, or contributions:

1. Check the troubleshooting section above
2. Review existing issues and documentation
3. Contact the development team at ari@aricummings.com

---

## Changelog

### Version 1.0.0 (Current)
- Initial release
- API key management (generate, validate, store)
- File operations (upload, download, list, search, delete)
- Cross-platform support
- Secure HTTPS communication
- Comprehensive error handling
- Match-based command routing

---

## Future Enhancements

Planned features for future releases:

- [ ] Batch file operations
- [ ] Progress bars for large transfers
- [ ] File metadata display (size, date)
- [ ] Configuration file support
- [ ] Shell completion scripts
- [ ] GUI application
- [ ] Scheduled uploads/downloads
- [ ] File encryption support
- [ ] Sync functionality

---

**Last Updated:** February 17, 2026



