/*!
为什么不能用 call

符号 `<<` 的优先级

内核态产生中断时，将上下文压栈

用户态产生中断时，将上下文保存在某页中

Rust 会在函数的开始和结尾加入一些额外的指令，控制栈寄存器等

指令相对寻址与虚拟内存

问题：
- 修改 epc

sscratch 寄存器：user stack 和 user context

# 指令
`sfence.vma`：清除 TLB 缓存

# TODO
Trap::Interrupt(SupervisorExternal) => {
    crate::board::irq_handler();
}

Trap::Exception(Exception::UserEnvCall) => {
    cx.inc_epc(4);
    cx.set_ret(
        trap::syscall::syscall(cx.syscall_id(), cx.fn_args()) as usize
    );
},

scratch 寄存器的作用
*/

use alloc::{ format, vec };
use riscv::{ addr::BitField, register::{ self, sscratch, stval } };
use ones::{
    concurrency::{process, thread},
    intervene::{ data::Data, Cause, Dependence, Lib as L },
    memory::Address,
    runtime::address_space::AddressSpace
};

use core::arch::global_asm;
global_asm!(include_str!("handler.S"));

pub struct Lib;

impl Dependence for Lib {
    fn cause() -> Cause {
        use register::scause;
        use Cause::*;

        let raw = scause::read();
        let class = raw.bits().get_bit(size_of::<usize>() * 8 - 1);
        let number = raw.code();

        
        let cause = if class { // interrupt
            match number {
                9 => External,
                _ => Unknown,
            }
        } else { // exception
            match number {
                3 => Breakpoint,
                8 => EnvCall,
                _ => Unknown,
            }
        };

        log(vec![
            format!("Cuase: {:?}", cause),
            format!("Class: {}", class),
            format!("Number: {}", number),
        ]);

        cause
    }

    #[inline]
    fn value() -> usize {
        stval::read()
    }

    #[inline]
    fn syscall(id: usize, args: [usize; 3]) -> isize {
        use crate::system_call;
        use ones::system_call::Lib;
        system_call::Handler::syscall(id, args)
    }

    fn breakpoiont(idata: &mut Data) {
        idata.cx.pc +=2;
    }
    
    fn envcall() {
        todo!()
    }

    #[inline]
    fn service_set(address: usize) {
        sscratch::write(address as usize);
    }

    #[inline]
    fn handler_set(address: usize) {
        use riscv::register::{ stvec, mtvec::TrapMode };
        unsafe{ stvec::write(address, TrapMode::Direct) };
    }

    #[inline]
    fn layout() -> (usize, usize, usize, usize) {
        extern "C" {
            fn handler_user();
            fn load_user_context();
            fn handler_kernel();
            fn load_kernel_context();
        }

        let relative = (
            0,
            load_user_context as usize - handler_user as usize,
            handler_kernel as usize - handler_user as usize,
            load_kernel_context as usize - handler_user as usize,
        );

        let base = Address::address(AddressSpace::itext().range.0);

        (base + relative.0, base + relative.1, base + relative.2, base + relative.3)
    }
}

impl L for Lib {
    fn init() {
        use register::sstatus; // sie
        
        let (_, _, handler_kernel, _) = Self::layout();
        Self::handler_set(handler_kernel);
        Self::service_set(Self::service_kernel as usize);

        unsafe {
            sstatus::set_sie(); // enable interrupt

            // sie::set_stimer(); // enable timer interrupt
        }
    } 

    fn service_user() {
        let (_, _, handler_kernel, _) = Self::layout();
        Self::handler_set(handler_kernel);
        thread::access(|scheduler| {
            let tid = scheduler.id.running.unwrap();
            let thread = scheduler.thread[tid].as_mut().unwrap();
            let cause = Self::cause();
            let value = Self::value();

            Self::dist_user(thread.idata(), cause, value);
        });
    }

    fn return_to_user() -> ! {
        use crate::intervene::register::sstatus;
        unsafe {
            sstatus::clear_sie();
        }
        let (handler_user, load_user_context, _, _) = Self::layout();
        Self::handler_set(handler_user);
        let (idata_addr, pid) = thread::access(|scheduler| {
            let tid = scheduler.id.running.unwrap();
            let thread = scheduler.thread[tid].as_mut().unwrap();
            let idata_addr = Address::address(AddressSpace::idata(tid).range.0);
            
            (idata_addr, thread.pid)
        });

        let satp = process::access(|manager| {
            let process = manager.process[pid].as_ref().unwrap();
            let page_table = process.page_table.root.number;

            1usize << 63 | page_table
        });

        unsafe {
            use core::arch::asm;
            asm!(
                "fence.i",
                "jr {load}",
                load = in(reg) load_user_context,
                in("a0") idata_addr,
                in("a1") satp,
                options(noreturn)
            )
        }
    }
}

use ones::info_module;
fn log<M>(msg: impl IntoIterator<Item = M>)
    where M: AsRef<str>,
{
    info_module::<M>("intervene", msg);
}

mod test {
    #![allow(unused)]

    pub fn main() {
        use riscv::asm::ebreak;

        unsafe { ebreak(); }
    }
}

mod config {
    // 0 => Exception::InstructionMisaligned,
    // 1 => Exception::InstructionFault,
    // 2 => Exception::IllegalInstruction,
    // 3 => Exception::Breakpoint,
    // 5 => Exception::LoadFault,
    // 6 => Exception::StoreMisaligned,
    // 7 => Exception::StoreFault,
    // 8 => Exception::UserEnvCall,
    // 10 => Exception::VirtualSupervisorEnvCall,
    // 12 => Exception::InstructionPageFault,
    // 13 => Exception::LoadPageFault,
    // 15 => Exception::StorePageFault,
    // 20 => Exception::InstructionGuestPageFault,
    // 21 => Exception::LoadGuestPageFault,
    // 22 => Exception::VirtualInstruction,
    // 23 => Exception::StoreGuestPageFault,
    // _ => Exception::Unknown,

    // 0 => Interrupt::UserSoft,
    // 1 => Interrupt::SupervisorSoft,
    // 2 => Interrupt::VirtualSupervisorSoft,
    // 4 => Interrupt::UserTimer,
    // 5 => Interrupt::SupervisorTimer,
    // 6 => Interrupt::VirtualSupervisorTimer,
    // 8 => Interrupt::UserExternal,
    // 9 => Interrupt::SupervisorExternal,
    // 10 => Interrupt::VirtualSupervisorExternal,
    // _ => Interrupt::Unknown,
}