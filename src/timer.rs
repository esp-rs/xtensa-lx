//! Xtensa internal timers

#[inline]
pub fn get_ccompare0() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare0 $0" : "=r"(x) ::: "volatile" ) };
    x
}
#[inline]
pub fn get_ccompare1() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare1 $0" : "=r"(x) ::: "volatile" ) };
    x
}
#[inline]
pub fn get_ccompare2() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare2 $0" : "=r"(x) ::: "volatile" ) };
    x
}

#[inline]
pub fn set_ccompare0(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare0 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}
#[inline]
pub fn set_ccompare1(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare1 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}
#[inline]
pub fn set_ccompare2(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare2 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}

/// Get the core cycle count
#[inline]
pub fn get_cycle_count() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccount $0" : "=r"(x) ::: "volatile" ) };
    x
}

/// cycle accurate delay using the cycle counter register
#[inline]
pub fn delay(clocks: u32) {
    let start = get_cycle_count();
    loop {
        if get_cycle_count().wrapping_sub(start) >= clocks {
            break;
        }
    }
}
