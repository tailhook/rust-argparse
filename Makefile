test: argparse_test
	./argparse_test

argparse_test: argparse/mod.rs
	rustc -o $@ --test $<

