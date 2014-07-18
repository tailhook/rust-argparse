LIBNAME := $(shell rustc --print-file-name argparse/mod.rs)


all: argparse-lib

test: argparse_test
	./argparse_test

argparse_test: argparse/mod.rs argparse/*.rs
	rustc -o $@ --test $<

argparse-lib: $(LIBNAME)

$(LIBNAME): argparse/mod.rs argparse/*.rs
	rustc -o $@ $<

examples: greeting structure

%: examples/%.rs $(LIBNAME)
	rustc -o $@ $< -L .

.PHONY: argparse-lib test examples
