
#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod uart;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    say_hello();
}

fn say_hello() -> ! {
    
    let message = "Hello from ARM64 bare metal!\r\n";
    uart::write_str(message);

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
