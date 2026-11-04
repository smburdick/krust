use core::ptr::{read_volatile, write_volatile};
use core::hint::spin_loop;

const UART_BASE: usize = 0x0900_0000; // QEMU VIRT_UART PL011 device

const UART_DR: *mut u32 = UART_BASE as *mut u32; // data reg
const UART_FR: *const u32 = (UART_BASE + 0x18) as *const u32; // flag reg

const UART_FR_TXFF: u32 = 1 << 5; // indication that fifo is full


pub fn write_byte(byte: u8) {

    unsafe {
        while read_volatile(UART_FR) & UART_FR_TXFF != 0 {
            spin_loop();
        }
        write_volatile(UART_DR, byte as u32);
    }
}

pub fn write_str(message: &str) {
    for byte in message.bytes() {
        write_byte(byte);
    }
}
