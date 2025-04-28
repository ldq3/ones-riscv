use ones::{
    concurrency::scheduler::inner::{ ModelScheduler, Scheduler as S },
    memory::{ page::Table, Flag }
};
use crate::{
    concurrency::process::Process,
    runtime::address_space::AddressSpace
};

pub struct Scheduler(pub ModelScheduler<Process>);

impl S for Scheduler {
    #[inline]
    fn new() -> Self {
        let inner = ModelScheduler::new();

        Self(inner)
    }

    fn new_process(&mut self, elf: &[u8]) {
        use ones::{
            memory::Address,
            concurrency::process::Process as _,
            runtime::address_space::AddressSpace as _,
            intervene::Lib
        };
        use crate::{ 
            cpu::satp,
            intervene::{ self },
        };

        let (ks_bound, ks_bottom) = self.0.alloc_kernel_stack();
        let kernel = &mut self.0.process[0];

        let mut process = Process::new(elf);

        kernel.0.address_space.0.page_table.map_area((ks_bound, ks_bottom), Flag::R | Flag::W);
        let ksp = Address::address(ks_bottom + 1) - 1;
        process.0.thread[0].0.context.sp = ksp;

        use crate::intervene::data::Data;
        use riscv::register::sstatus::{ self, SPP };

        let (page_number, _) = AddressSpace::intervene_data(0);
        let (frame_number, _) = process.0.address_space.0.page_table.get(page_number);
        let data = Data::get_mut(frame_number);

        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        data.status = sstatus.bits();
        
        data.kernel_info.addr_trans = satp(kernel.0.address_space.0.page_table.root());
        data.kernel_info.dist = intervene::Handler::service_user as usize;
        data.kernel_info.sp = ksp;

        let pid = process.id();
        self.0.process.insert(pid, process);
        self.0.ready.push_back((pid, 0));
    }
}
