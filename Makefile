all:

	mkdir -p bin

	clang \
		--target=aarch64-none-elf \
		-c src/boot.S \
		-o bin/boot.o

	rustc \
		--edition 2021 \
		--target aarch64-unknown-none \
		--crate-type staticlib \
		-C panic=abort \
		-C opt-level=2 \
		src/kernel.rs \
		-o bin/libkernel.a

	aarch64-elf-gcc \
		-nostdlib \
		-T src/linker.ld \
		-Wl,-Map=bin/kernel.map \
		bin/boot.o bin/libkernel.a \
		-o bin/kernel.elf

qemu: all
	qemu-system-aarch64 \
    -machine virt,gic-version=2 \
    -cpu cortex-a57 \
    -m 128M \
    -serial stdio \
    -display none \
    -monitor tcp:127.0.0.1:4444,server,nowait \
    -kernel bin/kernel.elf

clean:
	rm *.o *.elf *.map *.dtb *.dts *.a
	rm -rf bin/

dtb:
	qemu-system-aarch64 \
    -machine virt,dumpdtb=virt.dtb \
    -cpu cortex-a72
	dtc -I dtb -O dts virt.dtb > virt.dts
