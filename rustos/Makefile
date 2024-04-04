dev:
	cargo watch -x run

build-all:
	cargo build && cargo build --release

release:
	cargo build --release

os:
	qemu-system-x86_64 -drive format=raw,file=.\target\x86_64-rustos\debug\bootimage-rust_os.bin
