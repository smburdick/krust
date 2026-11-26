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
use crate::println;

pub static GLOBAL_TICK_CTR: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub extern "C" fn exception_handler(frame: &mut ExceptionFrame) -> () {

    let ec = (frame.esr >> 26) & 0x3f;
    match ec {
        0x0 => {
            println!("Invalid instruction.");
        }
        0x25 => { // faulty address
            let far_el1: u64;
            unsafe {
                unsafe {
                    asm!(
                        "mrs {}, FAR_EL1",
                        out(reg) far_el1,
                        options(nomem, nostack, preserves_flags)
                    );
                }
            }
            println!("Faulty address.");
            println!("FAR_EL1 = {:#010x}, ELS = {:#010x}, DFSC = {:#05x}", far_el1, frame.elr, frame.esr & 0x3F);
        },
        _ => {
            println!("Unexpected exception EC={:#x} ESR={:#x}", ec, frame.esr);
        },
   
    }
    unsafe {
        loop {
            asm!("wfe");
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
        // write_str("timer interrupt recv'd\r\n");
        start_physical_timer(62_500_000);
        GLOBAL_TICK_CTR.fetch_add(1, Ordering::Relaxed);
    } else if id == 33 {
        unsafe {
        while let Some(b) = try_read_byte() {
            RX.push(b);
            // Echo:
            // if b == b'\r' {
            //     write_str("\r\n");
            // } else {
            //     write_byte(b)
            // }
        }
        }
    } else {
        println!("unexpected IRQ: {:#x}", id);
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
