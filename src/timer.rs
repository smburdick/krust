use crate::asm;

pub fn timer_frequency() -> u64 {
    let frequency: u64;
    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            out(reg) frequency,
            options(nomem, nostack, preserves_flags)
        );
    }
    frequency
}

pub fn start_physical_timer(ticks: u64) {
    unsafe {
        // Mask the timer interrupt while configuring it
        asm!(
            "msr CNTP_CTL_EL0, {0}",
            in(reg) 2u64,
            options(nomem, nostack)
        );

        // Set the countdown
        asm!(
            "msr CNTP_TVAL_EL0, {0}",
            in(reg) ticks,
            options(nomem, nostack)
        );

        // Enable the timer and unmask its interrupt
        asm!(
            "msr CNTP_CTL_EL0, {0}",
            in(reg) 1u64,
            options(nomem, nostack)
        );
    }
}

pub fn stop_physical_timer() {
    unsafe {
        asm!(
            "msr CNTP_CTL_EL0, {0}",
            in(reg) 0u64,
            options(nomem, nostack)
        );
    }
}

pub fn read_physical_timer_control() -> u64 {
    let val: u64;
    unsafe {
        asm!(
            "mrs {0}, CNTP_CTL_EL0",
            out(reg) val,
            options(nomem, nostack)
        );
    }
    val
}
