use core::ptr::{read_volatile, write_volatile};
use core::hint::spin_loop;
use core::fmt::{self, Write};

const UART_BASE: usize = 0x0900_0000; // QEMU VIRT_UART PL011 device

const UART_DR: *mut u32 = UART_BASE as *mut u32; // data reg
const UART_FR: *const u32 = (UART_BASE + 0x18) as *const u32; // flag reg

const UART_FR_TXFF: u32 = 1 << 5; // indication that fifo is full

const UARTIMSC: *mut u32 = (UART_BASE + 0x38) as *mut u32;
const RXIM: u32 = 1 << 4;
const RXFE: u32 = 1 << 4;

pub struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}

pub fn write_byte(byte: u8) {

    unsafe {
        while read_volatile(UART_FR) & UART_FR_TXFF != 0 {
            spin_loop();
        }
        write_volatile(UART_DR, byte as u32);
    }
}

pub fn write_hex(num: u32) {
    write_str("0x");
    const HEX: &[u8] = b"0123456789ABCDEF";
    for i in (0..8).rev() {
        let nibble = ((num >> (i * 4)) & 0xF) as usize;
        write_byte(HEX[nibble]);
    }
}

pub fn write_str(message: &str) {
    for byte in message.bytes() {
        write_byte(byte);
    }
}

pub unsafe fn enable_rx_interrupt() {
    let mut reg: u32 = read_volatile(UARTIMSC);
    reg = reg | (RXIM as u32);
    write_volatile(UARTIMSC, reg);
}

pub unsafe fn try_read_byte() -> Option<u8> {
    let val = read_volatile(UART_FR);
    if val & RXFE != 0 {
        return None;
    }
    return Some((read_volatile(UART_DR) & 0xFF) as u8);
}
