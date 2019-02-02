# Needed SHELL since I'm using zsh
SHELL := /bin/bash

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: check
check: ## Check project
	cargo check

.PHONY: build
build: ## Build project
	cargo build

.PHONY: install
install: ## Install cargo-inspect on the local machine
	cargo install --force --path .

.PHONY: test
test: ## Run tests
	cargo test

.PHONY: publish
publish: ## Publish new version of cargo-inspect
	cargo publish