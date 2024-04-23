all: target/release/dewduct

.PHONY: all target/release/dewduct install

target/release/dewduct:
	cargo build --release

install:
	cargo install --path .
