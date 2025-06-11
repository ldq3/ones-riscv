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

use ones::intervene::Lib as _;

#[no_mangle]
pub fn kernel_main() -> ! {
    use ones::runtime::Lib as _;
    runtime::Lib::init();

    { // Initialize the intervene system.
        use ones::intervene::Lib as _;
        intervene::Lib::init();

        use ones::peripheral::plic;
        use crate::peripheral::config::{ HART_M, HART_S, INTERRUPT };

        use crate::runtime::config::PLIC_BASE;
        unsafe { plic::Handler::init(PLIC_BASE); }

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
        use ones::{
            file_system::Flag,
            concurrency::{ 
                process::Lib as _,
                thread::{ self, Lib as _ },

                coroutine::{ Lib as _, Coroutine, context::Context },
            }
        };
        let res = file_system::Handler::open_file("init", Flag::R_W);
        let pid = if let Some(mut file) = res {
            let elf = file.read_all();
            PLib::from_elf(None, &elf)
        } else { panic!() };

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
