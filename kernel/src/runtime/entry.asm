    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_bottom
    call kernel_main

    .section .bss.stack
    .space 4096 * 16 
boot_stack_bottom: 