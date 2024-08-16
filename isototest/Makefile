.PHONY: all install-pre-commit setup-hooks run-hooks format check clean help

all: build

help:
    @echo "Available targets:"
	@echo "    install-pre-commit - Install pre-commit"
	@echo "    setup-hooks - Set up pre-commit hooks"
	@echo "    run-hooks - Run pre-commit with the hooks provided"
	@echo "    build - Build the library in release mode"
	@echo "    build-log - Build the library in release mode with default-logging feature enabled"
	@echo "    build-dev - Build the library in development mode with default-log feature enabled"
	@echo "    docs - Build the library documentation and open it in the default browser"

install-pre-commit:
	@echo "Installing pre-commit..."
	pip install pre-commit || { echo 'Error installing pre-commit'; exit 1; }

setup-hooks:
	@echo "Setting up pre-commit hooks..."
	pre-commit install

run-hooks:
	@echo "Running hooks..."
	pre-commit run --all-files

setup: install-pre-commit setup-hooks
	@echo "Development environment set up successfully."

build:
	cargo build --lib --release

build-log:
	cargo build --lib --release --features default-logging

build-dev:
	cargo build --lib --features default-logging

docs:
	cargo doc --lib --no-deps --document-private-items --open --features default-logging

format:
	@echo "Formatting code with 'cargo fmt' ..."
	cargo fmt

check:
	@echo "Checking code with clippy..."
	cargo clippy
	cargo fmt -- --check

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Done."
