
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::arch::asm;

mod uart;
mod exception;
mod gic;
mod timer;

use uart::*;
use gic::*;
use timer::*;
use exception::*;

use core::sync::atomic::Ordering;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> () {

    write_str("Kernel booted.\n");

    unsafe {
        init_cpu_interface();
        init_distributor();
    }

    write_str("GIC initialized.\n");

    // Ensure that GICD registers are properly set up
    unsafe {
        assert_eq!((gicd_read_byte(GICD_IPRIORITYR + 30) as u32), 0x80);
        assert_eq!((gicd_read_byte(GICD_IPRIORITYR + 33) as u32), 0x80);
        assert_eq!((gicd_read_byte(GICD_ITARGETSR + 30) as u32), 0x1);
        assert_eq!((gicd_read_byte(GICD_ITARGETSR + 33) as u32), 0x1);
    }

    let freq = timer_frequency();
    write_str("Arming timer.\n");

    start_physical_timer(freq);

    unsafe {
        enable_rx_interrupt();
    }

    unmask_irq();

    let mut woke = false;
    
    loop {
        unsafe {
            asm!("wfi");
        }
        // write_str("Back in kernel main\n");
        // if !woke {
        //     write_str("WFI returned\n");
        //     woke = true;
        // }
        // let ticks = GLOBAL_TICK_CTR.load(Ordering::Relaxed);
        // write_str("Tick: ");
        // write_hex(ticks as u32);
        // write_str("\n");
        // write_str("Timer control: ");
        // write_hex(read_physical_timer_control() as u32);
        // write_str("\n");
    }

    unsafe {
        asm!("msr DAIFSet, #2"); // Mask IRQs
        asm!("msr CNTP_CTL_EL0, xzr"); // Disable physical timer
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }

}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // mask interrupts
    unsafe {
        asm!("msr DAIFSet, #0xf");
    }
    let _ = writeln!(uart::UartWriter, "{}", info);
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
