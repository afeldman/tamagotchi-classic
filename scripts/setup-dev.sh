#!/bin/bash
# Development environment setup for Tamagochi Classic

echo "🔧 Setting up Tamagochi Classic development environment..."

# Install Rust for embedded development
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add Rust embedded toolchain
rustup target add thumbv6m-none-eabi

# Install probe-rs for flashing
cargo install probe-rs --features cli

echo "✅ Development environment ready!"
echo "🚀 Next: cd firmware && cargo build"
