RUST=rustc
RFLAGS=-L deps/rust-sdl2/build/lib
VPATH=src

%: %.rs
	$(RUST) $(RFLAGS) $^ -o $@

eg01: eg01.rs

.DEFAULT: all
all: eg01
