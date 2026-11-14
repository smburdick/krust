// Boneyard for tests. use these later

fn test_freq() -> () {
    // 0x03B9ACA0 = 62.5 MHz
    let freq = timer_frequency();
    write_str("Timer frequency: ");
    write_hex(freq as u32);
    write_str(" Hz\r\n");
}

fn test_invalid_instruction() -> () {
    unsafe {
        asm!(".inst 0x0");
    }
    write_str("This shouldn't print!\n");
}

fn divide_by_zero(x: usize) -> () {
    let k = 1 / x;
}

fn text_exception() -> () {
    write_str("Before BRK\n");
    unsafe {
        asm!("movz x4, #0xBEEF", "movk x4, #0xDEAD, lsl #16", "brk #0", out("x4") _,); // breakpoint exception class 0x3c
    }
    write_str("After BRK\n");
}


fn say_hello() -> () {
    let mut uart = uart::UartWriter;
    let message = "\"All operating systems suck, but Linux just sucks less.\"\r\n---Linus Torvalds\r\n";
    let _ = writeln!(uart, "{}", message);
}


fn say_current_el() -> () {
    let current_el: u64;
    let mut uart = uart::UartWriter;
    unsafe {
        core::arch::asm!(
            "mrs {}, CurrentEL",
            out(reg) current_el,
            options(nomem, nostack, preserves_flags)
        );
    }
    writeln!(uart, "CurrentEL: {:#x}", current_el).unwrap(); // 0x4 = EL1
}
