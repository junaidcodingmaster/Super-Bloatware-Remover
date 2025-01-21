# Build Script Documentation

## Index
- [Supported Platforms](#supported-platforms)
- [Usage](#usage)
- [How to Use](#how-to-use)
- [What It Will Do](#what-it-will-do)

## Supported Platforms
The `build.sh` script supports the following Linux distributions:
- Ubuntu
- Debian
- Manjaro
- Arch
- AArch64
- Alpine
- Termux

Additionally, the script can build and install Rust applications for Windows from a Linux environment.

## Usage
```bash
./build.sh [OPTIONS]
```

### Options:
- `--build`              Build the Rust project.
- `--install`            Install the Rust project.
- `--build-for-win`      Build the Rust project for Windows.
- `--install-for-win`    Install the Rust project for Windows.
- `--os <OS_NAME>`       Manually specify the OS (e.g., ubuntu, debian, manjaro, arch, aarch64, alpine, termux).
- `--help`               Display help information.

## How to Use
1. **Make the script executable**:
   ```bash
   chmod +x build.sh
   ```

2. **Run the script with options**:
   - To build the Rust project:
     ```bash
     ./build.sh --build
     ```
   - To install the Rust project:
     ```bash
     ./build.sh --install
     ```
   - To build the Rust project for Windows:
     ```bash
     ./build.sh --build-for-win
     ```
   - To install the Rust project for Windows:
     ```bash
     ./build.sh --install-for-win
     ```
   - To manually specify the OS:
     ```bash
     ./build.sh --os ubuntu --build
     ```

3. **Get help information**:
   ```bash
   ./build.sh --help
   ```

## What It Will Do
The `build.sh` script automates the process of building and installing Rust applications. It performs the following tasks:

- **OS Detection**: Automatically detects the operating system or allows manual specification.
- **Build the Project**: Compiles the Rust project using `cargo build --release`.
- **Install the Project**: Installs the Rust project using `cargo install --path .`.
- **Cross-Compilation for Windows**: Builds and installs the Rust project for Windows from a Linux environment using the specified target.

---
If you like this script and my project then give a star to this project.

**Made By [Junaid](https://abujuni.dev)** 
