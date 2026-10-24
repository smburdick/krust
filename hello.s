.global _main
.align 2

.text
_main:
    // setup stack frame with 16-byte alignment
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    // Load addr of message using ADRP and add offset
    adrp x0, message@PAGE
    add x0, x0, message@PAGEOFF
    bl _printf

    // Restore stack and return 0
    mov x0, #0
    ldp x29, x30, [sp], #16
    ret

.data
message:
    .asciz "Hello, world!\n"
