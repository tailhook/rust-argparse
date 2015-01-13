all: libargparse.rlib

test: argparse_test
	./argparse_test

argparse_test: src/lib.rs src/*.rs
	rustc -o $@ --test $<

libargparse.rlib: src/lib.rs src/*.rs
	rustc -o $@ $<

examples: greeting structure

%: examples/%.rs $(LIBNAME)
	rustc -o $@ $< -L .

.PHONY: argparse-lib test examples
