LIB_RALLOC_DIR := $(PWD)/libralloc

LIB_RALLOC_SRC_DIR     := $(LIB_RALLOC_DIR)/src
LIB_RALLOC_INCLUDE_DIR := $(LIB_RALLOC_DIR)/include

LIB_RALLOC_BUILD_ALL_DIR     := $(LIB_RALLOC_DIR)/build
LIB_RALLOC_DEBUG_BUILD_DIR   := $(LIB_RALLOC_BUILD_ALL_DIR)/debug
LIB_RALLOC_RELEASE_BUILD_DIR := $(LIB_RALLOC_BUILD_ALL_DIR)/release

BUILD_TYPE ?= release

ifeq ($(BUILD_TYPE),debug)
LIB_RALLOC_BUILD_TYPE := Debug
export LIB_RALLOC_BUILD_DIR = $(LIB_RALLOC_DEBUG_BUILD_DIR)
else
LIB_RALLOC_BUILD_TYPE := Release
TASK_BUILD_RELEASE_OPTION := --release
export LIB_RALLOC_BUILD_DIR = $(LIB_RALLOC_RELEASE_BUILD_DIR)
endif

.PHONY: all
all: task

.PHONY: libralloc
libralloc:
	cmake $(LIB_RALLOC_DIR) -DCMAKE_BUILD_TYPE=$(LIB_RALLOC_BUILD_TYPE) -B$(LIB_RALLOC_BUILD_DIR)
	cmake --build $(LIB_RALLOC_BUILD_DIR)

.PHONY: generate_bindings
generate_bindings: libralloc
	bindgen $(LIB_RALLOC_INCLUDE_DIR)/ralloc.h -o src/ralloc/bindings.rs  --size_t-is-usize

.PHONY: task
task: generate_bindings
	cargo build $(TASK_BUILD_RELEASE_OPTION)

.PHONY: clean
clean:
	cargo clean
	rm -rf $(LIB_RALLOC_BUILD_ALL_DIR)
