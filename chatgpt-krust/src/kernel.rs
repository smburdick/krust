
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::arch::asm;

mod uart;
mod exception;

use uart::write_str;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> () {
    // say_hello();
    // say_current_el();

    // let mut uart = uart::UartWriter;
    // let spsel: u64;

    // unsafe {
    //     core::arch::asm!(
    //         "mrs {}, SPSel",
    //         out(reg) spsel
    //     );
    // }

    // writeln!(uart, "SPSel: {:#x}", spsel).unwrap();

    // text_exception();
    test_invalid_instruction();
    
    // loop {
    //     unsafe {
    //         asm!("wfe");
    //     }
    // }
}

fn test_invalid_instruction() -> () {
    unsafe {
        asm!(".inst 0x0");
    }
    write_str("This shouldn't print!\n");
}

fn divide_by_zero(x: usize) -> () {
    let k = 1 / x;
}

fn text_exception() -> () {
    write_str("Before BRK\n");
    unsafe {
        asm!("movz x4, #0xBEEF", "movk x4, #0xDEAD, lsl #16", "brk #0", out("x4") _,); // breakpoint exception class 0x3c
    }
    write_str("After BRK\n");
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
