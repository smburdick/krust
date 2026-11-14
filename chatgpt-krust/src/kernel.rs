
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::arch::asm;

mod uart;
mod exception;
mod gic;
mod timer;

use uart::{write_str, write_hex};
use gic::*;
use timer::*;
use exception::*;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> () {

    write_str("Kernel booted.\n");

    unsafe {
        init_cpu_interface();
        init_distributor();
    }

    write_str("GIC initialized.\n");

    let freq = timer_frequency();
    write_str("Arming timer.\n");

    start_physical_timer(freq);

    unmask_irq();
    
    loop {
        unsafe {
            asm!("wfi");
        }
        write_str("Back in kernel main\n");
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
