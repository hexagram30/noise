BIN_DIR = ./bin
BIN = target/release/noise

default: build

build: cli

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

cli: $(BIN_DIR)
	@cargo build --release
	@-cp -f $(BIN) $(BIN_DIR)/

run: cli
	@$(BIN)

run-help: cli
	@$(BIN) help

run-cave-help: cli
	@$(BIN) cave --help

examples:
	@cargo run --example=caves

.PHONY: examples
