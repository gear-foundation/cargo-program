.PHONY: all
all:
	@cargo build --release

.PHONY: cargo
cargo:
	@cargo install --path .
	@cargo program --help

.PHONY: install
install:
	@cargo install --path .

.PHONY: new
new:
	@cargo install --path .
	@rm -rf test-program
	@cargo program new test-program

.PHONY: pre-commit
pre-commit:
	@cargo fmt
	@cargo clippy -- -D warnings
	@cargo test

.PHONY: run
run:
	@cargo install --path .
	@cargo program --help
