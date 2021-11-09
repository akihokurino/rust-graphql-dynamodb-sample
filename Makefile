MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

clean:
	cargo clean

build:
	cargo build --bin api

run:
	cargo run --bin api