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
use crate::gic::*;
use crate::timer::*;
use core::sync::atomic::{AtomicU64, Ordering};

pub static GLOBAL_TICK_CTR: AtomicU64 = AtomicU64::new(0);

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

    let pending = unsafe { gicd_read(0x200) }; // GICD_ISPENDR0
    let active = unsafe { gicd_read(0x300) }; // GICD_ISACTIVER0

    // write_str("Pending: ");
    // write_hex((pending >> 30) & 1);
    // write_str("\r\n");

    // write_str("Active: ");
    // write_hex((active >> 30) & 1);
    // write_str("\r\n");

    if id >= 1020 {
        return;
    }
    if id == 30 {   
        // write_str("Timer control before rearm: ");
        // write_hex(read_physical_timer_control() as u32);
        // write_str("\r\n");

        start_physical_timer(62_500_000);
        // write_str("timer interrupt recv'd\r\n");
        GLOBAL_TICK_CTR.fetch_add(1, Ordering::Relaxed);
    } else if id == 33 {
        unsafe {
        while let Some(b) = try_read_byte() {
            if b == b'\r' {
                write_str("\r\n");
            } else {
                write_byte(b)
            }
        }
        }
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
