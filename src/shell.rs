use crate::uart::*;
use core::fmt::Write;
use crate::println;

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
        println!("you typed: {}", core::str::from_utf8(&self.buf[..self.len]).unwrap_or("?"));
        self.len = 0; // clear buffer
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
