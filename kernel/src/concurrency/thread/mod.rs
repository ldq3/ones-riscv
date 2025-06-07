pub mod context;

use ones::{
    concurrency::{
        process,
        thread::{ self, context::{ Context, Lib as _ }, Lib as L, Thread }
    },
    intervene::{ data::{ Data, KernelInfo }, Lib as _ },
    memory::{ page::Lib as _, Address, Flag },
};
use crate::{
    concurrency::thread::context::Lib as CLib,
    intervene,
    memory::page::Lib as PageLib
};

pub struct Lib;

impl L for Lib {
    fn new(pid: usize) -> usize {
        let (tid, segement) = Thread::new(pid, config::USER_STACK_SIZE, config::KERNEL_STACK_SIZE);
        
        process::access(|manager| {{
            use riscv::register::sstatus::{ self, SPP };

            let process = manager.process[pid].as_mut().unwrap();

            let mut status = sstatus::read();
            status.set_spp(SPP::User);
            let mut cx = Context::new(status.bits(), process.address_space.entry);

            PageLib::map_area(&mut process.page_table, segement[0].range, segement[0].flag | Flag::U);
            // PageLib::map(&mut process.page_table, segement[0].range.1 + 1, Flag::R | Flag::W);
            let sp = Address::address(segement[0].range.1 + 1) - 1;
            CLib::sp_set(&mut cx, sp);

            PageLib::map(&mut process.page_table, segement[1].range.0, segement[1].flag);
            let (frame_number, _) = PageLib::get(&mut process.page_table, segement[1].range.0);   
            
            let kernel = manager.process[0].as_mut().unwrap();

            PageLib::map(&mut kernel.page_table, segement[2].range.0, segement[2].flag);
            // PageLib::map(&mut kernel.page_table, segement[2].range.0 + 1, Flag::R | Flag::W);
            let isp = Address::address(segement[2].range.0 + 1) - 1;

            let addr_trans = { 
                use crate::satp;

                let process = manager.process[0].as_mut().unwrap();
                let frame_number = process.page_table.root.number;

                satp(frame_number)
            };
            let ki = KernelInfo { addr_trans, service: intervene::Lib::service_user as usize, sp: isp };
            let idata = Data{ cx, ki };

            thread::access(|scheduler| {
                let thread = scheduler.thread[tid].as_mut().unwrap();
                thread.idata = Address::address(frame_number);
                *thread.idata() = idata;
            });

            tid
        }});

        tid
    }
}

mod config {
    pub const KERNEL_STACK_SIZE: usize = 1;
    pub const   USER_STACK_SIZE: usize = 2;
}