target = native

FILES = $(shell find -name "*.c")
SKEWER = $(subst /,-, $(FILES))
OBJS = $(patsubst .-%,build/%,$(SKEWER))

# print: 
# 	@echo $(FILES)
# 	@echo $(SKEWER)
# 	@echo $(OBJS)


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


.PHONY: clean
clean:
	@rm -rf c_program
	@rm -rf build/*
	@rm -rf include/rust.h
	@rm -rf rust/target
	@clear


