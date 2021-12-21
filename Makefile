.PHONY: all check clean fmt fmt-check install linter pre-check pre-commit run test test-new test-new-async test-run test-run-node
all:
	@echo ------------ Build release ------------
	@cargo build --release

check:
	@echo ------------ Check ------------
	@cargo test

clean:
	@echo ------------ Clean ------------
	@rm -rf target

fmt:
	@echo ------------ Format ------------
	@cargo fmt

fmt-check:
	@echo ------------ Check format ------------
	@cargo fmt -- --check

linter:
	@echo ------------ Run linter ------------
	@cargo clippy -- -D warnings

install:
	@echo ------------ Install ------------
	@cargo install --path .

pre-check: fmt-check linter check test

pre-commit: fmt linter check

run: install
	@cargo program --help

test: test-new test-new-async

test-new: install
	@echo ------------ Test \`new\` ------------
	@rm -rf test-program
	@cargo program new test-program
	@cd test-program && cargo program build
	@test -f test-program/target/wasm32-unknown-unknown/debug/test_program.wasm
	@cd test-program && cargo program build --release
	@test -f test-program/target/wasm32-unknown-unknown/release/test_program.wasm

test-new-async: install
	@echo ------------ Test \`new --async\` ------------
	@rm -rf test-async-program
	@cargo program new test-async-program --async
	@cd test-async-program && cargo program build
	@test -f test-async-program/target/wasm32-unknown-unknown/debug/test_async_program.wasm
	@cd test-async-program && cargo program build --release
	@test -f test-async-program/target/wasm32-unknown-unknown/release/test_async_program.wasm

test-run: install
	@echo ------------ Test \`run\` ------------
	@rm -rf test-program
	@cargo program new test-program
	@cd test-program && cargo program run --release

test-run-node: install
	@echo ------------ Test \`run --node\` ------------
	@rm -rf test-program
	@cargo program new test-program
	@cd test-program && cargo program run --release --node "ws://localhost:9944"
