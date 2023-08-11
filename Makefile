# Makefile for the Rust project WAMI

TARGET = wami

.PHONY: all build install clean uninstall

all: build install clean

build:
	RUSTFLAGS="-C target-cpu=native" cargo build -r

install:
	cp target/release/wami /usr/local/bin/

clean:
	cargo clean

uninstall:
	sudo rm -f /usr/local/bin/wami
	sudo rm -Rf ~/.config/wami
