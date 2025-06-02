pub mod context;

use ones::{
    concurrency::{
        process,
        thread::{ context::{ Context, Lib as _ }, Lib as L, Thread }
    },
    intervene::{ data::{ Data, KernelInfo }, Lib as _ },
    memory::{ page::Lib as _, Address },
    runtime::address_space::AddressSpace
};
use crate::{
    concurrency::thread::context::Lib as CLib,
    intervene,
    memory::page::Lib as PageLib
};

pub struct Lib;

impl L for Lib {
    fn new(pid: usize) -> usize {
        let tid = process::access(|manager| {{
            use riscv::register::sstatus::{ self, SPP };

            let process = manager.process[pid].as_mut().unwrap();
            let mut status = sstatus::read();
            status.set_spp(SPP::User);
            let mut cx = Context::new(status.bits(), process.address_space.entry);

            let segement = process.address_space.stack(0);
            PageLib::map_area(&mut process.page_table, segement.range, segement.flag);
            let sp = Address::address(segement.range.1 + 1) - 1;
            CLib::sp_set(&mut cx, sp);

            let segement = AddressSpace::idata(1);
            PageLib::map(&mut process.page_table, segement.range.0, segement.flag);
            let (frame_number, _) = PageLib::get(&mut process.page_table, segement.range.0);   
            
            let segement = AddressSpace::istack(1);
            let kernel = manager.process[0].as_mut().unwrap();
            PageLib::map(&mut kernel.page_table, segement.range.0, segement.flag);
            let isp = Address::address(segement.range.1 + 1) - 4;

            let addr_trans = { 
                use crate::satp;

                let process = manager.process[0].as_mut().unwrap();
                let frame_number = process.page_table.root.number;

                satp(frame_number)
            };
            let ki = KernelInfo { addr_trans, service: intervene::Lib::service_user as usize, sp: isp };
            let idata = Data{ cx, ki };
            let tid = Thread::new(pid, frame_number, idata);

            tid
        }});

        tid
    }
}
