#!/bin/bash

# Install Rust with the nightly toolchain
echo "Installing Rust with the nightly toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# Source the Rust environment to use cargo and rustc immediately
source $HOME/.cargo/env

# Download libtorch
echo "Downloading libtorch..."
wget https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-2.1.0%2Bcpu.zip -O libtorch-2.1.0.zip


# Unzip libtorch
echo "Unzipping libtorch..."
unzip libtorch-2.1.0.zip

# Set LIBTORCH environment variable
export LIBTORCH=$(pwd)/libtorch
echo "LIBTORCH set to $LIBTORCH"

# Add the LIBTORCH environment variable to the user's profile for persistence
echo "export LIBTORCH=$LIBTORCH" >> $HOME/.bashrc
echo "export LIBTORCH=$LIBTORCH" >> $HOME/.profile
export LIBTORCH_BYPASS_VERSION_CHECK=1


# Verify Rust installation
echo "Verifying Rust installation..."
rustc --version

# Confirm setup is complete
echo "Setup complete! LIBTORCH has been installed and configured."
