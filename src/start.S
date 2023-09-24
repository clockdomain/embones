.section .init, "ax"
.global _start
_start:
   // Setup the global pointer to enable linker relaxation.
   // Linker relaxation enables generation of relative jump
   // instruction on function calls and jumps. The relative
   // jumps have a tighter encoding than absolute jumps hence
   // reducing code memory usage.
   .option push
   .option norelax
      la gp, __global_pointer$
   .option pop

    // Setup stack pointer   
    la sp, _sstack
    
    // Interrupts are disabled and will remain disabled in ROM so we only
    // need to worry about exceptions, so no reason to do vectored.
    // Setup Direct Exception Vector
    la t0, _exception_handler
    csrw mtvec, t0

    // Setup NMI Vector
    // Load address of NMI handler
    la t0, _nmi_handler
    // Load address of MMIO NMI vector register
    // CLP_SOC_IFC_REG_INTERNAL_NMI_VECTOR = 0x3003062c
    li t1, 0x3003062c
    // Store address of NMI handler in MMIO NMI vector register
    sw t0, 0x0(t1)


.section .init.text, "ax"
.align 2
_exception_handler:
    // Save sp to mscratch
    csrw mscratch, sp

    // Switch to exception stack
    la sp, _sestack

    // Allocate space for all relevant registers 
    // (ra, sp, a0-7, t0-6, mepc, mcause, mscause, mstatus, mtval)
    addi sp, sp, -88

    // Save relevant registers to stack except x2(sp) since that is in mscratch
    sw ra,  0x0(sp)
    // Skipping 0x4(sp) for now to store sp later
    sw a0, 0x8(sp)
    sw a1, 0xC(sp)
    sw a2, 0x10(sp)
    sw a3, 0x14(sp)
    sw a4, 0x18(sp)
    sw a5, 0x1C(sp)
    sw a6, 0x20(sp)
    sw a7, 0x24(sp)
    sw t0, 0x28(sp)
    sw t1, 0x2C(sp)
    sw t2, 0x30(sp)
    sw t3, 0x34(sp)
    sw t4, 0x38(sp)
    sw t5, 0x3C(sp)
    sw t6, 0x40(sp)

    // Save original sp to 0x4(sp)
    csrr t0, mscratch // Load mscratch (original sp) to t0
    sw t0, 0x4(sp)

    // Save mepc to 0x44(sp)
    csrr t0, mepc // Load mepc to t0
    sw t0, 0x44(sp)

    // Save mcause to 0x48(sp)
    csrr t0, mcause // Load mcause to t0
    sw t0, 0x48(sp)

    # // Save mscause to 0x4C(sp)
    // MSCAUSE = 0x7FF
    csrr t0, 0x7FF // Load mscause to t0
    sw t0, 0x4C(sp)

    // Save mstatus to 0x50(sp)
    csrr t0, mstatus // Load mstatus to t0
    sw t0, 0x50(sp)

    // Save mtval to 0x54(sp)
    csrr t0, mtval // Load mtval to t0
    sw t0, 0x54(sp)

    // Call the rust trap handler with the stack pointer as the parameter
    addi a0, sp, 0
    jal exception_handler

    // Restore relevant registers except x2(sp)
    lw ra,  0x0(sp)
    // Skipping 0x4(sp) for now to store sp later
    lw a0, 0x8(sp)
    lw a1, 0xC(sp)
    lw a2, 0x10(sp)
    lw a3, 0x14(sp)
    lw a4, 0x18(sp)
    lw a5, 0x1C(sp)
    lw a6, 0x20(sp)
    lw a7, 0x24(sp)
    lw t0, 0x28(sp)
    lw t1, 0x2C(sp)
    lw t2, 0x30(sp)
    lw t3, 0x34(sp)
    lw t4, 0x38(sp)
    lw t5, 0x3C(sp)
    lw t6, 0x40(sp)

    // Restore original sp from 0x4(sp)
    lw sp,  0x4(sp)

    mret
