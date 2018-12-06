.PHONY: build
build:
	cargo build

.PHONY: install
install:
	cargo install --force --path .

.PHONY: test
test:
	cargo test