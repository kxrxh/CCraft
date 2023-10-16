# Set the name of your Rust project
PROJECT_NAME = ccraft

# Set the target directory where the compiled executable will be placed
TARGET_DIR = target/release

# Set the installation directory for the executable
INSTALL_DIR = /usr/local/bin

TEST_DIR = testpy

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

# Install the executable to the user executable directory
install: build
	@echo "⚙️ Installing the executable..."
	@install -m 755 $(TARGET_DIR)/$(EXECUTABLE) $(INSTALL_DIR)
	@echo "✅ Installation completed!"
	@clean

# Run the tests
test: clean-tests build 
	@echo "🧪 Running tests..."
	@install -m 777 $(TARGET_DIR)/$(EXECUTABLE) $(TEST_DIR)
	cd $(TEST_DIR) && python tests.py

# Clean the project
clean:
	@echo "🧹 Cleaning the project..."
	@cargo clean
	@echo "✅ Clean completed!"

clean-tests:
	@echo "🧹 Cleaning up..."
	@find $(TEST_DIR) -mindepth 1 -maxdepth 1 ! -name 'tests.py' -exec rm -rf {} +

.PHONY: all build install clean