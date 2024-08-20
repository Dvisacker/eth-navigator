BINARY_NAME := eth-manager
INSTALL_PATH := /usr/local/bin

.PHONY: all build install uninstall

all: build install

build:
	@echo "Building release version..."
	@cargo build --release
	@echo "Build complete."

install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_PATH)"
	@mkdir -p $(INSTALL_PATH)
	@cp target/release/$(BINARY_NAME) $(INSTALL_PATH)/
	@echo "Installation complete. Please ensure $(INSTALL_PATH) is in your PATH."

uninstall:
	@echo "Uninstalling $(BINARY_NAME) from $(INSTALL_PATH)"
	@rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "Uninstallation complete."