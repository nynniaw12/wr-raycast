# vars
TARGET ?= native
FEATURE ?= raycasting
OUT_DIR := ./wasm

# Default target
.PHONY: all
all: build

# Build target (decides based on TARGET variable)
.PHONY: build
build:
ifeq ($(TARGET),native)
	cargo build --features $(FEATURE)
else ifeq ($(TARGET),web)
	rm -rf $(OUT_DIR)
	wasm-pack build --target web --out-dir $(OUT_DIR) -- --features $(FEATURE)
else
	$(error Unknown target "$(TARGET)", use 'native' or 'web')
endif

# Run target (decides based on TARGET variable)
.PHONY: run
run:
ifeq ($(TARGET),native)
	cargo run --features $(FEATURE)
else ifeq ($(TARGET),web)
	@echo "Cannot run a WebAssembly target directly. Serve the $(OUT_DIR) directory."
else
	$(error Unknown target "$(TARGET)", use 'native' or 'web')
endif

.PHONY: clean
clean:
	cargo clean
	rm -rf $(OUT_DIR)

