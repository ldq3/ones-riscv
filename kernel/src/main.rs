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
mod system_call;

use cpu::satp;
use ones::{ intervene::Lib, memory::Address };
use runtime::heap::Main as _;

#[no_mangle]
pub fn kernel_main() -> ! {
    use ones::cpu::Lib as _;

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

        // scheduler::Handler::init();
        use ones::concurrency::process::{ self, Lib as _ };
        use crate::concurrency::process::Lib as PLib;

        PLib::new_kernel();
        let kernel_satp = process::access(|manager| { 
            let process = manager.process[0].as_mut().unwrap();
            let frame_number = process.page_table.root.number;

            satp(frame_number)
        });

        cpu::Handler::page_enable(kernel_satp);
    }

    { // Initialize the intervene system.
        use ones::intervene::Lib as _;
        intervene::Lib::init();

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
        use crate::concurrency::{ 
            process::Lib as PLib,
            thread::Lib as TLib,
            coroutine::Lib as CLib,
        };
        use ones::concurrency::{ 
            process::Lib as _,
            thread::{ self, Lib as _ },
            coroutine::{ Lib as _, Coroutine, context::Context },
        };
        let pid = PLib::init();
        TLib::new(pid);

        let cx = thread::access(|scheduler|{
            let tid = scheduler.id.switch_s();

            let thread = scheduler.thread[tid].as_mut().unwrap();
            let sp = thread.idata().ki.sp;

            use crate::intervene;
            Context::new(intervene::Lib::return_to_user as usize, sp)
        });

        let _cid = Coroutine::new(cx);
        CLib::switch_to_ready();
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