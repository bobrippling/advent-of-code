.PHONY: util

util: util/permutations
	RUST_BACKTRACE=1 util/permutations

util/%: util/%.rs
	rustc -g --test $< -o $@
