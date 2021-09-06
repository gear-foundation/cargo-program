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

.PHONY: pre-commit
pre-commit:
	@cargo fmt
	@cargo clippy -- -D warnings
	@cargo test

.PHONY: run
run:
	@cargo install --path .
	@cargo program --help

.PHONY: test-new
test-new:
	@cargo install --path .
	@rm -rf test-program
	@cargo program new test-program
	@cd test-program && cargo program build
	@test -f test-program/target/wasm32-unknown-unknown/debug/test_program.wasm
	@cd test-program && cargo program build --release
	@test -f test-program/target/wasm32-unknown-unknown/release/test_program.wasm

.PHONY: test-new-async
test-new-async:
	@cargo install --path .
	@rm -rf test-async-program
	@cargo program new test-async-program --async
	@cd test-async-program && cargo program build
	@test -f test-async-program/target/wasm32-unknown-unknown/debug/test_async_program.wasm
	@cd test-async-program && cargo program build --release
	@test -f test-async-program/target/wasm32-unknown-unknown/release/test_async_program.wasm
