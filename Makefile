target = native

# I'm struggling with this part of the Makefile. 
# I want to take every *.c file in src/,
# turn its path into snake case, 
# and then make an object file in the build/ directory.
# So src/foo/math.c would become build/foo-math.o
# gcc -c src/foo/math.c -o build/foo-math.o

FILES  = $(shell find ./src -name "*.c" )
OBJS   = $(subst /,_, $(FILES))
OBJS  := $(patsubst ._src_%,build/%,$(OBJS))

run: rust_header c_program 
	./c_program

c_program: build/main.o build/release/librust.a
	gcc -O3 -march=$(target) $^ -o $@


dump_asm: build/main.o build/release/librust.a
	gcc -S -o -O3 $^  

build/main.o: src/main.c
	gcc -c $^ -o build/main.o

rust_header: ./rust/src/
	@cargo test -q --manifest-path ./rust/Cargo.toml > build/test_rust
	@cbindgen -l c ./rust > include/rust.h

build/release/librust.a: ./rust/src/
	@RUSTFLAGS="-Ctarget-cpu=$(target)" cargo build --release --manifest-path ./rust/Cargo.toml --target-dir ./build/

print: 
	@echo $(FILES)
	@echo $(OBJS)

.PHONY: clean
clean:
	@rm -rf c_program
	@rm -rf build/*
	@rm -rf include/rust.h
	@rm -rf rust/target
	@clear


