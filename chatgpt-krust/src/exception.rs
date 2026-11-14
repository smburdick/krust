#[repr(C)]
pub struct ExceptionFrame {
    pub x: [u64;31],
    pub sp: u64,
    pub elr: u64,
    pub spsr: u64,
    pub esr: u64,
}

use crate::uart::*;
use core::fmt::Write;
use core::arch::asm;
use gic::*;
use timer::*;


#[no_mangle]
pub extern "C" fn exception_handler(frame: &mut ExceptionFrame) -> () {

    let ec = (frame.esr >> 26) & 0x3f;

    match ec {
        0x0 => {
            write_str("Invalid instruction.\n");
            loop {}
        }
        0x3c => { // broken from exception_test()
            unsafe {
                if frame.x[4] == 0xDEADBEEF {
                    write_str("Yum!\n");
                } else {
                    write_str("Wheres the beef?!\n");
                }
                frame.elr += 4; // skip breakpoint
            }
        },
        _ => {
            write_str("Unexpected exception class.\n");
        }
    }
}

#[no_mangle]
pub extern "C" fn irq_handler_rs(frame: &mut ExceptionFrame) {
    let id = unsafe { acknowledge_interrupt() };
    if id >= 1020 {
        return;
    }
    if id == 30 {
        stop_physical_timer();
        write_str("timer interrupt recv'd\r\n");
    } else {
        write_str("unexpected IRQ: ");
        write_hex(id);
        write_str("\r\n"); // TODO: write a string formatter
    }
    unsafe { end_interrupt(id); }
}

pub fn unmask_irq() {
    unsafe {
        asm!(
            "msr DAIFClr, #2",
            options(nomem, nostack)
        );
    }
}
