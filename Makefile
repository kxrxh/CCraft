# Makefile

# Set the name of your Rust project
PROJECT_NAME = your_project_name

# Set the target directory where the compiled executable will be placed
TARGET_DIR = target/release

# Set the installation directory for the executable
INSTALL_DIR = /usr/local/bin

# Set the name of the executable
EXECUTABLE = $(PROJECT_NAME)

# Set the path to the Rust compiler (change this if necessary)
RUSTC = rustc

# Default target
all: build

# Build the project in release mode
build:
	@echo "🛠️ Building the project..."
	@cargo build --release
	@echo "✅ Build completed!"

# Install the executable to the user executable folder
install:
	@echo "⚙️ Installing the executable..."
	@install -m 755 $(TARGET_DIR)/$(EXECUTABLE) $(INSTALL_DIR)
	@echo "✅ Installation completed!"

# Clean the project
clean:
	@echo "🧹 Cleaning the project..."
	@cargo clean
	@echo "✅ Clean completed!"

.PHONY: all build install clean