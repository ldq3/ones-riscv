/*!
# Project Structure
标签：平台相关

程序运行环境：
- 标准库依赖
- 语义项（language item）
- 内存管理

工具：
- 日志

中断（平台相关）

同步

虚拟化：
- 进程
- 系统调用
- CPU
- 内存

外设

文件系统

# 初始化
运行时
内存管理
介入
*/
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod logger;

mod runtime;
mod memory;
mod concurrency;
mod cpu;
mod intervene;
mod peripheral;
mod file_system;

use cpu::satp;
use ones::{ file_system::Flag, memory::Address };
use runtime::heap::Main as _;

#[no_mangle]
pub fn kernel_main() -> ! {
    use ones::{
        concurrency::scheduler::Main as _,
        cpu::Lib as _ ,
    };

    { // Initialize the runtime.
        use ones::memory::clear;
        extern "C" {
            fn sbss();
            fn ebss();
        }
        unsafe{ clear(sbss as usize, ebss as usize) };

        use runtime::heap;
        heap::Handler::init();

        logger::init();

        extern "C" {
            fn ekernel();
        }
        use ones::memory::page::frame::Frame;
        let head = Address::number(ekernel as usize);
        let tail = Address::number(config::END);
        Frame::init(head, tail);

        use crate::concurrency::scheduler;
        use ones::memory::page::Table;
        scheduler::Handler::init(); 

        let kernel_satp = scheduler::Handler::access(|scheduler| {
            let process = &mut scheduler.process[0];
            let frame_number = process.0.address_space.0.page_table.root();

            satp(frame_number)
        });

        cpu::Handler::page_enable(kernel_satp);
    }

    { // Initialize the intervene system.
        use ones::intervene::Lib as _;
        intervene::Handler::init();

        use ones::peripheral::plic;
        use crate::peripheral::config::{ HART_M, HART_S, INTERRUPT };

        unsafe { plic::Handler::init(config::PLIC_BASE); }

        plic::Handler::threshold(HART_M, 1);
        plic::Handler::threshold(HART_S, 0);
    
        for (interrupt, priority) in INTERRUPT {
            plic::Handler::enable(HART_S, interrupt);
            plic::Handler::priority(interrupt, priority);
        }
    }

    use ones::file_system::Main as _;
    { // Initialize the file system.
        use crate::{ peripheral::disk, file_system };

        file_system::Handler::init(disk::HANDLER.clone());
    }
    
    { // User program.
        let file = file_system::Handler::open_file("init", Flag::R_W);
        if let Some(mut file) = file {
            let elf_data = file.read_all();
            use concurrency::scheduler;
            scheduler::Handler::new_process(&elf_data);
            scheduler::Handler::switch_to_ready();
        } else { panic!("Error when opening the init_process."); }
    }

    panic!("Shutdown machine!");
}

mod config {
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