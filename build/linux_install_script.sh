#!/bin/bash
# This installation script is part of the Ryujin-cli project.
# It is intended to be used to install the project from a GitLab repository on a Linux system.
# This script will:
# 1. Check if Rust and Cargo are installed. If not, it will install them.
# 2. Check if Docker is installed. If not, it will install it.
# 3. Check if Git is installed. If not, it will install it.
# 4. Clone the project from the GitLab repository.
# 5. Build the project.
# 6. Clean up the cloned directory.
# The script will exit if any of the dependencies are not met or if any of the steps fail.
# The script will suggest adding the binary to the PATH after successful installation.
# The script can be run with an optional argument to specify the directory where the app will be installed.
# If no argument is provided, the app will be installed to the current directory.
# The script assumes that the user has sudo privileges and that the system is running a Debian-based Linux distribution.

USER_HOME=$(eval echo ~${SUDO_USER})
script_path=$(readlink -f "$BASH_SOURCE")
script_dir=$(dirname "$script_path")



# Function to clean up the cloned directory
clean_up() {
    echo "Cleaning up..."
    # Move the binary, README, services, and conf directories to the parent directory
    mv target/release/ryujin-cli ..
    mv README.md ..
    mv services ..
    mv conf ..
    cd ..
    # Remove the cloned directory as we don't need it anymore
    rm -rf git-repo
}

# Define the GitLab repository URL
REPO_URL="https://gitlab.com/ryujingroup/ryujin-cli.git"
# Define where to clone the repo based on argument
if [ -z "$1" ]; then
    CLONE_DIR="./ryujin-cli/git-repo"
else
    CLONE_DIR="$1/ryujin-cli/git-repo"
fi
# Check if the directory already exists
if [ -d "$CLONE_DIR" ]; then
    echo "The directory $CLONE_DIR already exists. Please remove it or choose a different directory."
    exit 1
fi
# Clone the project
echo "Cloning the project from GitLab..."
git clone $REPO_URL $CLONE_DIR
# Check if the clone was successful
if [ ! -d "$CLONE_DIR" ]; then
    echo "Failed to clone the repository."
    exit 1
fi
cd $CLONE_DIR
# Build the project
echo "Building the project..."
rustup default stable
cargo build --release > build.log
# Check if the build was successful
if [ ! -f "target/release/ryujin-cli" ]; then
    echo "Build failed."
    # save the build information for debugging purposes
    mkdir ../debug
    mv ./target/.rustc_info.json ../debug
    mv ./build.log ../debug
    clean_up
    exit 1
fi
# Clean up the cloned directory
clean_up
# Set path to the dir where the binary is located as a env variable
# It is needed so that the program can find the services and conf directories
RYUJIN_CLI_PATH="$script_dir/ryujin-cli"
# check if variable needs to be added to the .bashrc file or .zshrc file
if [ -f ~/.bashrc ]; then
    echo "Setting the path to the binary as an environment variable in the .bashrc file..."
    echo "export RYUJIN_CLI_PATH=\"$RYUJIN_CLI_PATH\"" >> $USER_HOME/.bashrc
    echo "Adding the binary to the PATH..."
    echo "export PATH=\"\$PATH:\$RYUJIN_CLI_PATH\"" >> $USER_HOME/.bashrc
    # Source the .bashrc file to update the current shell
    source $USER_HOME/.bashrc
elif [ -f ~/.zshrc ]; then
    echo "Setting the path to the binary as an environment variable in the .zshrc file..."
    echo "export RYUJIN_CLI_PATH=\"$RYUJIN_CLI_PATH\"" >> ~/.zshrc
    echo "Adding the binary to the PATH..."
    echo "export PATH=\"\$PATH:\$RYUJIN_CLI_PATH\"" >> ~/.zshrc
    # Source the .zshrc file to update the current shell
    source $USER_HOME/.zshrc
else
    echo "Failed to set the path to the binary as an environment variable in the .bashrc or .zshrc file."
    exit 1
fi
echo "Installation completed successfully."
echo "You will now need to open a new shell to use the command ryujin-cli."