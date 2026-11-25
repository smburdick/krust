use crate::uart::*;
use core::fmt::Write;
use crate::println;
use crate::exception::*;
use core::sync::atomic::Ordering;
use core::ptr::{read_volatile};
use crate::utils::*;

const BUF_LEN: usize = 128;

pub struct Shell {
    buf: [u8; BUF_LEN], // line being typed
    len: usize // how many bytes of buf are in use
}

impl Shell {
    pub fn new() -> Self {
        Self { buf: [0; BUF_LEN], len: 0 }
    }
    pub fn execute(&mut self) {
        let line = core::str::from_utf8(&self.buf[..self.len]).unwrap_or("?");
        let mut tokens = line.split_whitespace();
        let Some(cmd) = tokens.next() else { self.escape(); return; };
        match cmd {
            "help" => {
                println!("The available commands are:\n");
                println!("\tticks (print GLOBAL_TICK_COUNTER)");
                println!("\tel (print current exception level)");
                println!("\tpeek <addr> (inspect addr)");
                println!("\tpoke <addr> <val> (set address)");
                println!();
            },
            "ticks" => {
                println!("{:?}", GLOBAL_TICK_CTR.load(Ordering::Relaxed));
            },
            "el" => {
                let current_el: u64;
                unsafe {
                    core::arch::asm!(
                        "mrs {}, CurrentEL",
                        out(reg) current_el,
                        options(nomem, nostack, preserves_flags)
                    );
                }
                println!("EL{:x}", current_el >> 2);
            },
            "peek" => {
                match tokens.next().and_then(parse_hex) {
                    Some(addr) => {
                        if addr % 4 != 0 {
                            println!("Address {:#010x} is not word-aligned.", addr);
                            self.escape();
                            return;
                        }
                        let val = unsafe { read_volatile(addr as *const u32) };
                        println!("{:#010x}: {:#010x}", addr, val);
                    },
                    _ => {
                        println!("usage: peek <addr>");
                        self.escape();
                        return;
                    }
                }
            }
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
        self.escape();
    }
    fn escape(&mut self) {
        self.len = 0;
        self.prompt();
    }
    pub fn prompt(&self) {
        write_str("krust> ");
    }
    pub fn handle_byte(&mut self, b: u8) {
        match b {
            0x20..=0x7E => { // printable
                if self.len < self.buf.len() {
                    self.buf[self.len] = b;
                    self.len += 1;
                    write_byte(b);
                } else { // too many characters
                    write_byte(0x07); // bell
                }
            },
            0x7F | 0x08 => { // backspace
                if self.len > 0 {
                    self.len -= 1;
                    write_str("\x08 \x08");
                }
            },
            b'\r' => { // enter
                write_str("\r\n");
                self.execute();
            },
            _ => {}
        }
    }
}
