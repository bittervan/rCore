.PHONY: all
all:
	cargo build --release

strip: all
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/rcore -O binary target/riscv64gc-unknown-none-elf/release/os.bin

run: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

debug: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
		-s -S

gdb:
	riscv64-unknown-elf-gdb \
		-ex 'file target/riscv64gc-unknown-none-elf/release/rcore' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'


