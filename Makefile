.PHONY: test run

test:
	@clear && RUSTFLAGS=-Awarnings cargo test -q

run:
	@clear && RUSTFLAGS=-Awarnings cargo run -q -- test/test.c
