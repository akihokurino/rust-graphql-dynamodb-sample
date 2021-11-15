MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

clean:
	cargo clean
	rm -rf target_lambda

build:
	cd api && cargo build

deploy:
	docker build -t lambda_builder .
	docker run -it --rm -v ~/.cargo/registry:/root/.cargo/registry:z -v $(PWD):/build:z lambda_builder
	sam deploy

run:
	cd api && SSM_PARAMETER=/sample/dotenv cargo run