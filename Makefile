NAME=emoji
CARGO=cargo
DENO=deno

.PHONY: test build run fmt init generate

default: run

test:
	$(CARGO) test

run:
	$(CARGO) run

fmt:
	$(CARGO) fmt

build: generate
	$(CARGO) build --release --locked -v

generate:
	$(DENO) run --allow-all ./scripts/build_emoji_map.js

init:
	rustup component add rustfmt