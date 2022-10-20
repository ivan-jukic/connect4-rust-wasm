# https://www.gnu.org/software/make/manual/html_node/Special-Variables.html
# https://ftp.gnu.org/old-gnu/Manuals/make-3.80/html_node/make_17.html
PROJECT_MKFILE_PATH     := $(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
PROJECT_MKFILE_DIR      := $(shell cd $(shell dirname $(PROJECT_MKFILE_PATH)); pwd)

PROJECT_NAME			:= connect4-wasm
PROJECT_UI				:= connect4-ui
PROJECT_ROOT            := $(PROJECT_MKFILE_DIR)

UI_PORT					:= 8000
UI_BUILD_DIR			:= $(PROJECT_ROOT)/.build-ui
WASM_BUILD_DIR			:= $(PROJECT_ROOT)/.build-wasm
WASM_CODE_DIR			:= $(PROJECT_ROOT)/$(PROJECT_NAME)
WASM_CACHE_DIR			:= $(PROJECT_ROOT)/target
PARCEL_CACHE_DIR		:= $(PROJECT_ROOT)/.parcel-cache




# Code
# ===================================

review:
	cargo clippy

test:
	cargo test --lib

check-fmt:
	cargo fmt --check

# Starts the UI
start:
	yarn parcel $(PROJECT_UI)/**/*.html --port=$(UI_PORT) --dist-dir=$(UI_BUILD_DIR)

clean:
	rm -rf $(UI_BUILD_DIR) $(WASM_BUILD_DIR) $(WASM_CACHE_DIR) $(PARCEL_CACHE_DIR)


# Build WASM
# ===================================

wasm-build:
	wasm-pack build 						\
			  --release						\
			  --target web 					\
			  --out-dir $(WASM_BUILD_DIR)	\
			  $(WASM_CODE_DIR)

wasm-build-dev:
	wasm-pack build 						\
			  --dev							\
			  --target web 					\
			  --out-dir $(WASM_BUILD_DIR)	\
			  $(WASM_CODE_DIR)
