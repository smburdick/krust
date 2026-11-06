
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
mod uart;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    say_hello();
    say_current_el();
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}


fn say_hello() -> () {
    let mut uart = uart::UartWriter;
    let message = "\"All operating systems suck, but Linux just sucks less.\"\r\n---Linus Torvalds\r\n";
    let _ = writeln!(uart, "{}", message);
}


fn say_current_el() -> () {
    let current_el: u64;
    let mut uart = uart::UartWriter;
    unsafe {
        core::arch::asm!(
            "mrs {}, CurrentEL",
            out(reg) current_el,
            options(nomem, nostack, preserves_flags)
        );
    }
    writeln!(uart, "CurrentEL: {:#x}", current_el).unwrap(); // 0x4 = EL1
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
