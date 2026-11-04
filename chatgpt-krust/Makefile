all:
	clang \
		--target=aarch64-none-elf \
		-c boot.S \
		-o boot.o

	rustc \
		--target aarch64-unknown-none \
		--crate-type staticlib \
		-C panic=abort \
		-C opt-level=2 \
		--emit=obj \
		kernel.rs \
		-o kernel.o

	aarch64-elf-gcc \
		-nostdlib \
		-T linker.ld \
		-Wl,-Map=kernel.map \
		boot.o kernel.o \
		-o kernel.elf

qemu: all
	qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -m 128M \
    -serial stdio \
    -display none \
    -monitor tcp:127.0.0.1:4444,server,nowait \
    -kernel kernel.elf

clean:
	rm *.o *.elf *.map *.dtb *.dts

dtb:
	qemu-system-aarch64 \
    -machine virt,dumpdtb=virt.dtb \
    -cpu cortex-a72
	dtc -I dtb -O dts virt.dtb > virt.dts
