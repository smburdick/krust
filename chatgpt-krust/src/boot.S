
.section .text.boot // subsequent instructions go in this section
.global _start // globally visible symbol

_start:
    // Clear BSS
    ldr x0, =__bss_start
    ldr x1, =__bss_end

clear_bss:
    cmp x0, x1
    b.hs bss_done
    str xzr, [x0], #8
    b clear_bss

bss_done:

    msr SPSel, #1
    isb

    // Set up our stack.
    ldr x0, =_stack_top // load stack top cst into x0
    mov sp, x0 // set sp = x0

    // Setup interrupt vector table
    adrp x0, vectors
    add x0, x0, :lo12:vectors

    msr VBAR_EL1, x0
    isb

    // Call our Rust kernel.
    bl kernel_main // branch link to kernel_main function

hang:
    wfe // wait for event
    b hang // infinite recursion?

.section .bss // global/static data
.align 12 // 2^12 byte boundary

_stack_bottom:
    .skip 4096 * 4 // reserve 16 KiB
_stack_top: // gives us the above data for the stack

.section .text.vectors, "ax"
.balign 2048

.global vectors
vectors:

.macro vector_entry handler
    b \handler
    .space 124
.endm

    // Current EL, using SP_EL0
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler

    // Current EL, using SP_ELx
    vector_entry sync_handler
    vector_entry irq_handler
    vector_entry default_handler
    vector_entry default_handler

    // Lower EL, AArch64
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler

    // Lower EL, AArch32
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler
    vector_entry default_handler

.global sync_handler
sync_handler:
    // allocate exception frame
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    // Capture exception state
    add x0, sp, #288
    str x0, [sp, #248]
    mrs x0, ELR_EL1
    str x0, [sp, #256]
    mrs x0, SPSR_EL1
    str x0, [sp, #264]
    mrs x0, ESR_EL1
    str x0, [sp, #272]

    // Pass exception frame to rust
    mov x0, sp
    bl exception_handler

    // Restore system registers
    ldr x0, [sp, #256]
    msr ELR_EL1, x0
    ldr x0, [sp, #264]
    msr SPSR_EL1, x0

    // Restore general purpose registers
    ldp x1,  x2,  [sp, #8]
    ldp x3,  x4,  [sp, #24]
    ldp x5,  x6,  [sp, #40]
    ldp x7,  x8,  [sp, #56]
    ldp x9,  x10, [sp, #72]
    ldp x11, x12, [sp, #88]
    ldp x13, x14, [sp, #104]
    ldp x15, x16, [sp, #120]
    ldp x17, x18, [sp, #136]
    ldp x19, x20, [sp, #152]
    ldp x21, x22, [sp, #168]
    ldp x23, x24, [sp, #184]
    ldp x25, x26, [sp, #200]
    ldp x27, x28, [sp, #216]
    ldp x29, x30, [sp, #232]

    // Restore x0 and release exception frame
    ldr x0, [sp, #0]     // Restore x0 from the frame
    add sp, sp, #288 

    // Return from exception
    eret

.global irq_handler
irq_handler:
    // Allocate exception frame
    sub sp, sp, #288

    // Save general-purpose registers
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    // Capture exception state
    add x0, sp, #288
    str x0, [sp, #248]

    mrs x0, ELR_EL1
    str x0, [sp, #256]

    mrs x0, SPSR_EL1
    str x0, [sp, #264]

    // ESR_EL1 isn't needed for IRQ classification
    str xzr, [sp, #272]

    // Pass exception frame to Rust
    mov x0, sp
    bl irq_handler_rs

    // Restore system registers
    ldr x0, [sp, #256]
    msr ELR_EL1, x0
    ldr x0, [sp, #264]
    msr SPSR_EL1, x0

    // Restore general-purpose registers
    ldp x1,  x2,  [sp, #8]
    ldp x3,  x4,  [sp, #24]
    ldp x5,  x6,  [sp, #40]
    ldp x7,  x8,  [sp, #56]
    ldp x9,  x10, [sp, #72]
    ldp x11, x12, [sp, #88]
    ldp x13, x14, [sp, #104]
    ldp x15, x16, [sp, #120]
    ldp x17, x18, [sp, #136]
    ldp x19, x20, [sp, #152]
    ldp x21, x22, [sp, #168]
    ldp x23, x24, [sp, #184]
    ldp x25, x26, [sp, #200]
    ldp x27, x28, [sp, #216]
    ldp x29, x30, [sp, #232]

    // Restore x0 and release exception frame
    ldr x0, [sp, #0]
    add sp, sp, #288

    eret

.global default_handler
default_handler:
    wfe
    b default_handler


// TODO: DRY he exception handlers.
/**


alloc_exception: // (TODO: PROLOGUE)
    // Allocate exception frame
    sub sp, sp, #288

    // Save general-purpose registers
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    // Capture exception state
    add x0, sp, #288
    str x0, [sp, #248]

    mrs x0, ELR_EL1
    str x0, [sp, #256]

    mrs x0, SPSR_EL1
    str x0, [sp, #264]

    // ESR_EL1 isn't needed for IRQ classification
    str xzr, [sp, #272]

    // Pass exception frame to Rust
    mov x0, sp

**/
