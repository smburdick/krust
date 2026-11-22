pub const GICD_BASE: usize = 0x800_0000;
pub const GICC_BASE: usize = 0x801_0000;
pub const GICC_PMR: usize = 0x004;
pub const GICC_IAR: usize = 0x00C;
pub const GICC_EOIR: usize = 0x010;

pub const GICD_TYPER: usize = 0x004;
pub const GICD_CTLR: usize = 0x000;
pub const GICD_ISENABLER: usize = 0x100;
pub const GICD_IPRIORITYR: usize = 0x400;
pub const GICD_ITARGETSR: usize = 0x800;

pub const UART0_IRQ: u32 = 33;

use core::ptr::{read_volatile, write_volatile};

pub unsafe fn gicd_write(offset: usize, value: u32) {
    let reg = (GICD_BASE + offset) as *mut u32;
    write_volatile(reg, value);
}

unsafe fn gicd_write_byte(offset: usize, value: u8) {
    let reg = (GICD_BASE + offset) as *mut u8;
    write_volatile(reg, value);
}

pub unsafe fn gicc_write(offset: usize, value: u32) {
    let reg = (GICC_BASE + offset) as *mut u32;
    write_volatile(reg, value);
}

pub unsafe fn gicc_read(offset: usize) -> u32 {
    let reg = (GICC_BASE + offset) as *const u32;
    read_volatile(reg)
}

pub unsafe fn gicd_read(offset: usize) -> u32 {
    let reg = (GICD_BASE + offset) as *const u32;
    read_volatile(reg)
}

pub unsafe fn gicd_read_byte(offset: usize) -> u8 {
    let reg = (GICD_BASE + offset) as *const u8;
    read_volatile(reg)
}

pub unsafe fn init_distributor() {
    // timer (PPI)
    set_priority(30, 0x80); // assign the EL1 physical timer a priority
    enable_interrupt(30);
    // UART (SPI)
    set_priority(UART0_IRQ, 0x80);
    set_target(UART0_IRQ, 0x01);
    enable_interrupt(UART0_IRQ);
     // enable interrupt forwarding -- set bit 0 of GICD_CTLR
    gicd_write(GICD_CTLR, 1);
}

pub unsafe fn enable_interrupt(id: u32) {
    let register_offset = GICD_ISENABLER + 4 * (id as usize / 32);
    let bit = 1u32 << (id % 32);
    gicd_write(register_offset, bit);
}

pub unsafe fn set_priority(id: u32, priority: u8) {
    let offset = GICD_IPRIORITYR + id as usize;
    gicd_write_byte(offset, priority);
}

pub unsafe fn init_cpu_interface() {
    gicc_write(GICC_PMR, 0xFF); // allow interrupts of all priorities
    gicc_write(0x000, 1); // enable interrupt signaling to the processor
}

pub unsafe fn acknowledge_interrupt() -> u32 {
    gicc_read(GICC_IAR) & 0x3FF
}

pub unsafe fn end_interrupt(id: u32) {
    gicc_write(GICC_EOIR, id);
}

pub unsafe fn set_target(id: u32, cpu_mask: u8) {
    let offset = GICD_ITARGETSR + id as usize;
    gicd_write_byte(offset, cpu_mask);
}
