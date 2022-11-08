BIN_DIR = ./bin
BIN = target/release/noise

default: deps build

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

build: cli

deps:
	@cargo update

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

test:
	@cargo test

cli: $(BIN_DIR)
	@cargo build --release
	@rm -f $(BIN_DIR)/*
	@cargo install --path . --root .

run: cli
	@$(BIN)

run-help: cli
	@$(BIN) help

run-cave-help: cli
	@$(BIN) cave --help

examples:
	@cargo run --example=ascii-caves
	@cargo run --example=ascii-land-sea
	@cargo run --example=ascii-levels
	@cargo run --example=caves
	@cargo run --example=levels

clean:
	@cargo clean
	@rm -rf example_images/*

.PHONY: examples
