/*!
file entry.asm under every architecture subfolder is needed. 初始化栈
*/

pub mod lang_items;
pub mod heap;
pub mod address_space;

use alloc::{ vec, format };
use ones::{
    concurrency::process::{ self, Lib as _ },
    memory::Address,
    runtime::{ Lib as L, address_space::AddressSpace },
};
use crate::concurrency::process::Lib as PLib;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

pub struct Lib;

impl L for Lib {
    fn init() {
        use ones::memory::clear;
        extern "C" {
            fn stext();
            fn etext();

            fn srodata();
            fn erodata();

            fn sdata();
            fn edata();

            fn kernel_stack();
    
            fn sbss();
            fn ebss();

            fn ekernel();
        }
        unsafe{ clear(sbss as usize, ebss as usize) };

        use heap::Main as _;
        heap::Handler::init();
        
        use crate::logger;
        logger::init();

        use ones::memory::page::frame::Frame;
        let head = Address::number(ekernel as usize);
        let tail = Address::number(config::END);
        Frame::init(head, tail);

        let text = (Address::number(stext as usize), Address::number(etext as usize));
        let read_only_data = (Address::number(srodata as usize), Address::number(erodata as usize));
        let data = (Address::number(sdata as usize), Address::number(edata as usize));
        let static_data = (Address::number(kernel_stack as usize), Address::number(ebss as usize));
        let frame_data = (Address::number(ekernel as usize), Address::number(config::END));

        info(vec![
            format!("Segement text: {:x} - {:x}", text.0, text.1),
            format!("Segement read only data: {:x} - {:x}", read_only_data.0, read_only_data.1),
            format!("Segement data: {:x} - {:x}", data.0, data.1),
            format!("Segement static data: {:x} - {:x}", static_data.0, static_data.1),
            format!("Segement frame: {:x} - {:x}", frame_data.0, frame_data.1)
        ]);

        use crate::runtime::config::MMIO;
        let address_space = AddressSpace::new_kernel(stext as usize, MMIO, text, read_only_data, data, static_data, frame_data);
        
        PLib::new_kernel(address_space);

        use crate::cpu;
        use ones::cpu::Lib as _;
        
        let kernel_satp = process::access(|manager| { 
            let process = manager.process[0].as_mut().unwrap();
            let frame_number = process.page_table.root.number;

            cpu::satp(frame_number)
        });
        cpu::Lib::page_enable(kernel_satp);
    }
}

use ones::info_module;
fn info<M>(msg: impl IntoIterator<Item = M>)
    where M: AsRef<str>,
{
    info_module::<M>("runtime", msg);
}

pub mod config {
    // memory
    /**
    单位：页

    元素：(head, tail)
    */
    pub const MMIO: &[(usize, usize)] = &[
        (   0x100,    0x102), // VIRT_TEST/RTC in virt machine
        ( 0x2_000,  0x2_010), // core local interrupter (CLINT)
        ( 0xc_000,  0xc_210), // VIRT_PLIC in virt machine
        (0x10_000, 0x10_009), // VIRT_UART0 with GPU  in virt machine
    ];

    pub const PLIC_BASE: usize =  0xc_000_000;
    // pub const UART_BASE: usize = 0x10_000_000;

    // pub const BASE:      usize = 0x80_200_000;
    /// 单位：字节（byte）
    pub const END :      usize = 0x88_000_000;
}