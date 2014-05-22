RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC = $(shell find . -type f -name '*.rs')

RUSTFLAGS += --cfg ext_image
RUSTFLAGS += --cfg ext_ttf

all: libsdl2.dummy

libsdl2.dummy: lib.rs $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $<
	touch $@

test: test~
	./test~

test~: lib.rs $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) --cfg test --test $< -o $@

clean:
	rm -f *.dylib *.rlib *.dummy *.o *.bc test~
	rm -rf *.dSYM
