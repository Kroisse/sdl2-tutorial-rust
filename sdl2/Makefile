RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC = $(shell find . -type f -name '*.rs')

RUSTFLAGS += --cfg ext_image
RUSTFLAGS += --cfg ext_ttf

all: libsdl2.dummy

libsdl2.dummy: sdl2.rc $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
	touch $@

test: test~
	./test~

test~: sdl2.rc $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) --cfg test --test $< -o $@

clean:
	rm -f *.dylib *.dummy test~
	rm -rf *.dSYM
