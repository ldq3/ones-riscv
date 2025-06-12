.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text.itext
    .globl handler_user
    .globl load_user_context
    .globl handler_kernel
    .globl load_kernel_context

    .align 2
handler_user:
    csrrw sp, sscratch, sp 
_save_context:
    # save general
    # skip x0, constant 0
    sd x1, 1*8(sp)
    # skip x2(sp), the sp is in sscratch
    sd x3, 3*8(sp)
    # skip x4(tp), application does not use it
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # save csr
    csrr t0, sstatus
    sd t0, 32*8(sp)
    csrr t1, sepc
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
_switch_to_kernel:
    ld t0, 34*8(sp) # load kernel_satp into t0
    ld t1, 36*8(sp) # load trap_handler into t1
    ld sp, 35*8(sp) # move to kernel_sp
    # switch to kernel space
    csrw satp, t0
    sfence.vma
_distribute:
    # ditribute and handle exception
    jr t1 # call distribute
load_user_context:
_switch_back:
    # call get_kernel_context
    # a0: *TrapContext in user space(Constant); a1: user space token
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0
    # now sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)

    # return
    sret

    .align 2
handler_kernel:
    addi sp, sp, -34*8 
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    mv a0, sp
    csrr t2, sscratch
    jalr t2
load_kernel_context:
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8

    # return
    sret