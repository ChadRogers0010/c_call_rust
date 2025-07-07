# https://makefiletutorial.com/#makefile-cookbook
# Thanks to Job Vranish (https://spin.atomicobject.com/2016/08/26/makefile-c-projects/)

# Requires Cargo and Cbindgen to be installed
# Uses 'find' shell command

# Name of program
TARGET_EXEC := program
# Platform to compile for
COMPILATION_TARGET := native

SRC_DIRS    := ./src
BUILD_DIR   := ./build
INCLUDE_DIR := ./include

# Find all the C and C++ files we want to compile
# Note the single quotes around the * expressions. The shell will incorrectly expand these otherwise, but we want to send the * directly to the find command.
SRCS := $(shell find $(SRC_DIRS) -name '*.cpp' -or -name '*.c' -or -name '*.s')

# Prepends BUILD_DIR and appends .o to every src file
# As an example, ./your_dir/hello.cpp turns into ./build/./your_dir/hello.cpp.o
OBJS := $(SRCS:%=$(BUILD_DIR)/%.o)

# String substitution (suffix version without %).
# As an example, ./build/hello.cpp.o turns into ./build/hello.cpp.d
DEPS := $(OBJS:.o=.d)

# Every folder in ./src will need to be passed to GCC so that it can find header files
INC_DIRS := $(shell find $(SRC_DIRS) -type d)
# Add a prefix to INC_DIRS. So moduleA would become -ImoduleA. GCC understands this -I flag
INC_FLAGS := $(addprefix -I,$(INC_DIRS))

# The -MMD and -MP flags together generate Makefiles for us!
# These files will have .d instead of .o as the output.
CPPFLAGS := $(INC_FLAGS) -MMD -MP -O3 -march=$(COMPILATION_TARGET)

# C/++ compilers
CC  := gcc
CXX := g++

RUST_NAME     := rust
RUST_DIR      := ./$(RUST_NAME)
RUST_SRC      := $(RUST_DIR)/src
RUST_TOML     := $(RUST_DIR)/Cargo.toml
RUST_HEADER   := $(INCLUDE_DIR)/$(RUST_NAME).h
RUST_ARTIFACT := $(BUILD_DIR)/release/lib$(RUST_NAME).a

RUSTC_FLAGS   := --crate-type=staticlib --manifest-path=$(RUST_TOML) --target-dir=$(BUILD_DIR) --release -- -C target-cpu=$(COMPILATION_TARGET)
CARGO_FLAGS   :=  

build: $(BUILD_DIR)/$(TARGET_EXEC) 

run: $(BUILD_DIR)/$(TARGET_EXEC)
	./$<

# The final build step.
$(BUILD_DIR)/$(TARGET_EXEC): $(RUST_HEADER) $(RUST_ARTIFACT) $(OBJS) 
	gcc $(OBJS) $(RUST_ARTIFACT) -o $@ $(LDFLAGS)

# Build step for C source
$(BUILD_DIR)/%.c.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CPPFLAGS) $(CFLAGS)-c $< -o $@

# Build step for C++ source
$(BUILD_DIR)/%.cpp.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS)-c $< -o $@

# Build step for Rust header
$(RUST_HEADER): $(RUST_SRC)
	@mkdir -p $(INCLUDE_DIR) $(BUILD_DIR)
	cargo test -q $(RUST_FLAGS) --manifest-path $(RUST_TOML)
	cbindgen -l c $(RUST_DIR) -o $(RUST_HEADER)

# Build step for Rust source
$(RUST_ARTIFACT): $(RUST_SRC) $(RUST_HEADER)
	@mkdir -p $(BUILD_DIR)
	cargo rustc $(RUSTC_FLAGS) $(CARGO_FLAGS)

# Generate header for librust.a
.PHONY: rust.h
rust.h: $(RUST_HEADER)

.PHONY: clean
clean:
	rm include/rust.h
	rm -rf $(BUILD_DIR)/*

# Include the .d makefiles. The - at the front suppresses the errors of missing
# Makefiles. Initially, all the .d files will be missing, and we don't want those
# errors to show up.
-include $(DEPS)
