.altmacro

.set REG_SIZE, 8

.set CONTEXT_SIZE, 34


.macro SAVE reg, offset
    sd \reg, \offset * REG_SIZE(sp)
.endm


.macro SAVE_N n
    SAVE x\n, \n
.endm


.macro LOAD reg, offset
    ld \reg, \offset * REG_SIZE(sp)
.endm


.macro LOAD_N n
    LOAD x\n, \n
.endm


    .section .text
    .globl __interrupt
__interrupt:
    csrrw   sp, sscratch, sp

    addi    sp, sp, -CONTEXT_SIZE * REG_SIZE

    SAVE    x1, 1

    csrr    x1, sscratch
    SAVE    x1, 2

    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr


    csrr    t0, sstatus
    csrr    t1, sepc
    SAVE    t0, 32
    SAVE    t1, 33


    mv      a0, sp

    csrr    a1, scause

    csrr    a2, stval

    jal     handle_interrupt


    .globl __restore
__restore:
    mv      sp, a0

    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0 
    csrw    sepc, t1

    addi    t0, sp, CONTEXT_SIZE * REG_SIZE
    csrw    sscratch, t0

    LOAD    x1, 1
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr

    LOAD    x2, 2
    sret 

