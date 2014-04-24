test: argparse_test
	./argparse_test

argparse_test: argparse/mod.rs argparse/*.rs
	rustc -o $@ --test $<

