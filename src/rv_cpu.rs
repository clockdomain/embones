/// Trap Record
#[repr(C)]
pub(crate) struct TrapRecord {
    /// Return Address
    pub ra: u32,
    /// Stack Pointer
    pub sp: u32,
    /// Callee Saved Registers
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    /// Temporary Registers
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    /// Machine Exception Program Counter
    pub mepc: u32,
    /// Machine Exception Cause
    pub mcause: u32,
    /// Machine Exception Program Counter
    pub mscause: u32,
    /// Machine Status Register
    pub mstatus: u32,
    /// Machine Trap Value
    pub mtval: u32,
}
