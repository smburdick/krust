
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let uart = 0x0900_0000 as *mut u8;

    let message = b"Hello from ARM64 bare metal!\r\n";

    unsafe {
        for byte in message {
            uart.write_volatile(*byte);
        }
    }

    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

