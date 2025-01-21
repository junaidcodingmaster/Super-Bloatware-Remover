#!/bin/bash

# Function to display usage information
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  --build              Build the Rust project"
    echo "  --install            Install the Rust project"
    echo "  --build-for-win      Build the Rust project for Windows"
    echo "  --install-for-win    Install the Rust project for Windows"
    echo "  --os <OS_NAME>      Manually specify the OS (e.g., ubuntu, debian, manjaro, arch, aarch64, alpine, termux)"
    echo
    echo "For help, use: $0 --help"
}

# Function to display help information
help() {
    usage
    echo
    echo "This script checks the operating system and performs build and install operations for a Rust application."
    echo "It supports building and installing for both Linux and Windows from a Linux environment."
}

# Function to check the OS
check_os() {
    if [[ -n "$MANUAL_OS" ]]; then
        OS_NAME=$MANUAL_OS
    elif [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS_NAME=$ID
    else
        echo "Unsupported OS"
        exit 1
    fi

    case "$OS_NAME" in
        ubuntu|debian|manjaro|arch|aarch64|alpine|termux)
            echo "Detected OS: $OS_NAME"
            ;;
        *)
            echo "Unsupported Linux distribution: $OS_NAME"
            exit 1
            ;;
    esac
}

# Function to build the Rust project
build() {
    echo "Building the Rust project..."
    cargo build --release
    if [[ $? -ne 0 ]]; then
        echo "Build failed."
        exit 1
    fi
    echo "Build completed successfully."
}

# Function to install the Rust project
install() {
    echo "Installing the Rust project..."
    cargo install --path .
    if [[ $? -ne 0 ]]; then
        echo "Installation failed."
        exit 1
    fi
    echo "Installation completed successfully."
}

# Function to build for Windows
build_for_win() {
    echo "Building the Rust project for Windows..."
    cargo build --release --target=x86_64-pc-windows-gnu
    if [[ $? -ne 0 ]]; then
        echo "Windows build failed."
        exit 1
    fi
    echo "Windows build completed successfully."
}

# Function to install for Windows
install_for_win() {
    echo "Installing the Rust project for Windows..."
    cargo install --path . --target=x86_64-pc-windows-gnu
    if [[ $? -ne 0 ]]; then
        echo "Windows installation failed."
        exit 1
    fi
    echo "Windows installation completed successfully."
}

# Main script logic
if [[ $# -eq 0 ]]; then
    echo "Error: No options provided."
    usage
    exit 1
fi

# Initialize MANUAL_OS variable
MANUAL_OS=""

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --build)
            build
            shift
            ;;
        --install)
            install
            shift
            ;;
        --build-for-win)
            build_for_win
            shift
            ;;
        --install-for-win)
            install_for_win
            shift
            ;;
        --os)
            shift
            if [[ -z "$1" ]]; then
                echo "Error: No OS name provided after --os."
                usage
                exit 1
            fi
            MANUAL_OS=$1
            shift
            ;;
        --help)
            help
            exit 0
            ;;
        *)
            echo "Error: Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Check the OS after parsing arguments
check_os
