
.section .text.boot // subsequent instructions go in this section
.global _start // globally visible symbol

_start:
    // Set up our stack.
    ldr x0, =_stack_top // load stack top cst into x0
    mov sp, x0 // set sp = x0

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
