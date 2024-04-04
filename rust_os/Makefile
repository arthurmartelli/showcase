dev:
	cargo watch -x run

build:
	cargo bootimage --release

os:
	cargo bootimage && qemu-system-x86_64 -drive format=raw,file=.\target\x86_64-rust_os\debug\bootimage-rust_os.bin

os-release:
	cargo bootimage --release && qemu-system-x86_64 -drive format=raw,file=.\target\x86_64-rust_os\release\bootimage-rust_os.bin

