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
	@rm -rf test_program
	@cargo program new test_program

.PHONY: run
run:
	@cargo run --release -- --help
