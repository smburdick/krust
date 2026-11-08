
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
    vector_entry default_handler
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
    mrs x2, ESR_EL1

    // Extract EC = ESR_EL1[31:26]
    lsr x3, x2, #26

    // Print the low hexadecimal digit
    and x3, x3, #0xf

    cmp x3, #9
    b.le numeric

    add x3, x3, #('A' - 10)
    b print

numeric:
    add x3, x3, #'0'

print:
    ldr x0, =0x09000000
    str w3, [x0]

1:
    wfe
    b 1b

.global default_handler
default_handler:
    wfe
    b default_handler

