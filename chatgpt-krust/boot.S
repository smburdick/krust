
.section .text.boot
.global _start

_start:
    // Set up our stack.
    ldr x0, =_stack_top
    mov sp, x0

    // Call our Rust kernel.
    bl kernel_main

hang:
    wfe
    b hang

.section .bss
.align 12

_stack_bottom:
    .skip 4096 * 4
_stack_top:

