RUST=rustc
RFLAGS=-L deps/rust-sdl2/build/lib
VPATH=src

%: %.rs
	$(RUST) $(RFLAGS) $^ -o $@

.DEFAULT: all
all: eg01 eg02 eg03 eg04

.PHONY: clean
clean:
	rm -f ./eg*
