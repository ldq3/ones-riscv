/*！
ttext 节的位置不能动

# SV 39
RV 64

开启虚拟内存后：
有效物理内存地址为 56 位
虚拟内存地址为 64 位，但仅低 39 位有效，共分为三层页表

satp 寄存器的组成：

| 位域 | 位数 | 描述                                                                 |
|------|------|----------------------------------------------------------------------|
| MODE | 1-3  | 地址转换模式：值为 1 表示使用 Sv39 模式                       |
| ASID | 4-11 | 地址空间标识符（Address Space Identifier），用于区分不同的地址空间 |
| PPN  | 12-63| 物理页号（Physical Page Number），指向顶级页表的物理地址           |
*/
pub mod data_registers;
pub mod timer;

use ones::cpu::Lib;

pub struct Handler;

impl Lib for Handler {
    fn shutdown(failure: bool) -> ! {
        #[allow(deprecated)]
        use sbi_rt::{ system_reset, NoReason, Shutdown, SystemFailure };

        if !failure {
            system_reset(Shutdown, NoReason);
        } else {
            system_reset(Shutdown, SystemFailure);
        }

        unreachable!() 
    }

    #[inline]
    fn page_enable(bits: usize) {
        use riscv::register::satp;
        use core::arch::asm;

        unsafe {
            satp::write(bits);
            asm!("sfence.vma");
        }
    }

    #[inline]
    fn plic_enable() {
        use riscv::register::sie;
        unsafe { sie::set_sext(); }
    }
}

#[inline]
pub fn satp(frame_number: usize) -> usize {
    general_satp(Mode::Sv39, frame_number)
}

#[inline]
fn general_satp(mode: Mode, frame_number: usize) -> usize {
    (mode as usize) << 60 | frame_number
}

#[allow(unused)]
pub enum Mode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
}