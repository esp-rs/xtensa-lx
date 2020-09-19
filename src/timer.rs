//! Xtensa internal timers

#[cfg(feature = "ccompare0")]
#[inline]
pub fn get_ccompare0() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare0 $0" : "=r"(x) ::: "volatile" ) };
    x
}
#[cfg(feature = "ccompare1")]
#[inline]
pub fn get_ccompare1() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare1 $0" : "=r"(x) ::: "volatile" ) };
    x
}
#[cfg(feature = "ccompare2")]
#[inline]
pub fn get_ccompare2() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare2 $0" : "=r"(x) ::: "volatile" ) };
    x
}

#[cfg(feature = "ccompare3")]
#[inline]
pub fn get_ccompare3() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare3 $0" : "=r"(x) ::: "volatile" ) };
    x
}

#[cfg(feature = "ccompare0")]
#[inline]
pub fn set_ccompare0(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare0 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}
#[cfg(feature = "ccompare1")]
#[inline]
pub fn set_ccompare1(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare1 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}
#[cfg(feature = "ccompare2")]
#[inline]
pub fn set_ccompare2(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare2 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}
#[cfg(feature = "ccompare3")]
#[inline]
pub fn set_ccompare3(val: u32) {
    unsafe {
        llvm_asm!("
        wsr.ccompare3 $0
        isync
        " :: "r"(val):::: "volatile" )
    };
}

/// Get the core cycle count
#[cfg(feature = "ccount")]
#[inline]
pub fn get_cycle_count() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccount $0" : "=r"(x) ::: "volatile" ) };
    x
}

/// cycle accurate delay using the cycle counter register
#[cfg(feature = "ccount")]
#[inline]
pub fn delay(clocks: u32) {
    let start = get_cycle_count();
    loop {
        if get_cycle_count().wrapping_sub(start) >= clocks {
            break;
        }
    }
}
