# Makefile for the Rust project WAMI

TARGET = wami

.PHONY: all build install clean uninstall

all: install clean

build:
	RUSTFLAGS="-C target-cpu=native" cargo build -r

install:
	cargo install --path .

clean:
	cargo clean

uninstall:
	cargo uninstall
	rm -Rf $HOME/.config/wami