# List all recipes
default:
	just --list --unsorted

# Lint
lint:
	cargo clippy -- --deny "warnings"
	cargo fmt -- --check
