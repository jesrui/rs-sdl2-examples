RUST=rustc
RFLAGS=-L deps/rust-sdl2/build/lib
VPATH=src

%: %.rs
	$(RUST) $(RFLAGS) $^ -o $@

.DEFAULT: all
all: eg01 eg02

eg01: eg01.rs

.PHONY: clean
clean:
	rm -f ./eg*
