//! Xtensa internal timers


pub fn get_ccompare0() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare0 $0" : "=r"(x) ::: "volatile" ) };
    x
}
pub fn get_ccompare1() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare1 $0" : "=r"(x) ::: "volatile" ) };
    x
}
pub fn get_ccompare2() -> u32 {
    let x: u32;
    unsafe { llvm_asm!("rsr.ccompare2 $0" : "=r"(x) ::: "volatile" ) };
    x
}

pub fn set_ccompare0(val: u32){
    unsafe { llvm_asm!("
        wsr.ccompare0 $0
        isync
        " :: "r"(val):::: "volatile" ) 
    };
}
pub fn set_ccompare1(val: u32){
    unsafe { llvm_asm!("
        wsr.ccompare1 $0
        isync
        " :: "r"(val):::: "volatile" ) 
    };
}
pub fn set_ccompare2(val: u32){
    unsafe { llvm_asm!("
        wsr.ccompare2 $0
        isync
        " :: "r"(val):::: "volatile" ) 
    };
}