OUTPUT_ARCH(riscv)
ENTRY(start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    .text : {
        stext = .;
        *(.text.entry)
        . = ALIGN(4K);
        ttext = .;
        *(.text.ttext)
        . = ALIGN(4K);
        *(.text .text.*)
        etext = .;
    }

    . = ALIGN(4K);
    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        erodata = .;
    }

    . = ALIGN(4K);
    .data : {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        edata = .;
    }

    . = ALIGN(4K);
    .bss : {
        kernel_stack = .;
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }

    . = ALIGN(4K);
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}