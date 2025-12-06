# Variables
CARGO = cargo
APP_NAME = guessing-game # Change this to your binary name if needed

# Default goal: runs when you type 'make'
.DEFAULT_GOAL := help

.PHONY: all
all: fmt lint test build ## Run formatting, linting, testing, and build

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# --- Development ---

.PHONY: check
check: ## Fast check to see if code compiles (no binary built)
	@$(CARGO) check

.PHONY: build
build: ## Build the project in debug mode
	@$(CARGO) build

.PHONY: run
run: ## Run the project
	@$(CARGO) run

.PHONY: watch
watch: ## Watch file changes and run the project (requires cargo-watch)
	@$(CARGO) watch -x run

# --- Quality Control ---

.PHONY: test
test: ## Run all tests
	@$(CARGO) test

.PHONY: fmt
fmt: ## Format code using rustfmt
	@$(CARGO) fmt

.PHONY: lint
lint: ## Run clippy linter with strict warnings
	@$(CARGO) clippy -- -D warnings

.PHONY: doc
doc: ## Generate documentation
	@$(CARGO) doc --open

# --- Release ---

.PHONY: release
release: ## Build the project for release (optimized)
	@$(CARGO) build --release

# --- Maintenance ---

.PHONY: clean
clean: ## Remove the target directory
	@$(CARGO) clean

.PHONY: update
update: ## Update dependencies
	@$(CARGO) update