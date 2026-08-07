# Makefile for typst-sheet-music (Scorify)

CARGO ?= cargo
TYPST ?= typst

WASM_TARGET := wasm32-unknown-unknown
WASM_BUILD_DIR := wasm/target/$(WASM_TARGET)/release
WASM_OUT := scorify_wasm.wasm

FONTS_DIR := fonts
ROOT_DIR := .

# Find all .typ files in examples/ and tests/
EXAMPLE_SRCS := $(shell find examples -type f -name '*.typ' 2>/dev/null)
TEST_SRCS    := $(shell find tests -type f -name '*.typ' 2>/dev/null)

EXAMPLE_PDFS := $(EXAMPLE_SRCS:.typ=.pdf)
TEST_PDFS    := $(TEST_SRCS:.typ=.pdf)
ALL_PDFS     := $(EXAMPLE_PDFS) $(TEST_PDFS)

WASM_SRCS    := $(shell find wasm/src -type f -name '*.rs' 2>/dev/null) wasm/Cargo.toml

.PHONY: all wasm pdfs examples tests clean help

all: wasm pdfs
	@echo "All Typst files compiled successfully."

wasm: $(WASM_OUT)

$(WASM_OUT): $(WASM_SRCS)
	@echo "Building WASM plugin..."
	cd wasm && $(CARGO) build --target $(WASM_TARGET) --release
	cp $(WASM_BUILD_DIR)/scorify_wasm.wasm $(WASM_OUT)

pdfs: $(ALL_PDFS)

examples: $(EXAMPLE_PDFS)

tests: $(TEST_PDFS)

%.pdf: %.typ $(WASM_OUT) lib.typ
	@echo "Compiling $<..."
	$(TYPST) compile "$<" "$@" --font-path $(FONTS_DIR) --root $(ROOT_DIR)

clean:
	@echo "Cleaning generated files..."
	rm -f $(ALL_PDFS)
	rm -f $(WASM_OUT)
	if [ -d wasm ]; then cd wasm && $(CARGO) clean 2>/dev/null || true; fi

help:
	@echo "Available targets:"
	@echo "  all       - Build WASM plugin and compile all Typst examples/tests (default)"
	@echo "  wasm      - Build WASM plugin ($(WASM_OUT))"
	@echo "  pdfs      - Compile all Typst files in examples/ and tests/"
	@echo "  examples  - Compile only example Typst files"
	@echo "  tests     - Compile only test Typst files"
	@echo "  clean     - Remove built WASM binary and generated PDF files"
