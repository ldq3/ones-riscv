pub mod thread;

use alloc::vec::Vec;
use ones::{
    concurrency::{process::{ thread::Thread as _, Dependence, ModelProcess, Process as P }, scheduler::Main}, memory::{ page::Table, Flag}, runtime::address_space::AddressSpace as _, Allocator
};

use crate::runtime::address_space::AddressSpace;
use thread::Thread;

pub struct Process(pub ModelProcess<Thread, AddressSpace>);

impl Dependence for Process {
    fn kernel_map_area(range: (usize, usize), flag: Flag) {
        use crate::concurrency::scheduler;
        scheduler::Handler::access(|scheduler| {
            let kernel = &mut scheduler.process[0];
            kernel.0.address_space.0.page_table.map_area(range, flag);
        })
    }
}

impl P for Process {
    fn new(elf_data: &[u8]) -> Self {
        let pid = Self::new_pid(); 

        use ones::runtime::address_space::AddressSpace as _;
        use crate::runtime::address_space::AddressSpace;
        let mut address_space = AddressSpace::from_elf(elf_data);

        use ones::Allocator;
        let allocator = Allocator::new(1, 15).unwrap();
        
        let mut vector = Vec::new();
        {
            use ones::concurrency::process::thread::Thread;
            use ones::intervene::context::UserContext;

            let tid = 0;

            let ksp = Self::alloc_kernel_stack();

            let usp = address_space.new_stack(tid);

            let frame_number = address_space.new_intervene(tid);

            use crate::{
                cpu::satp,
                intervene::{ self, Data }
            };
            let data = Data::get_mut(frame_number);
            data.user_context.init(address_space.0.entry, usp);
            use crate::concurrency::scheduler;
            let frame_num = scheduler::Handler::access(|scheduler| {
                let process = &mut scheduler.process[0];

                process.0.address_space.0.page_table.root()
            });
            data.kernel_info.addr_trans = satp(frame_num);
            data.kernel_info.dist = intervene::Handler::service_user as usize;
            data.kernel_info.sp = ksp;
 
            use ones::intervene::Lib as _;
            use crate::intervene::Handler;
            let thread = Thread::new(pid, tid, ksp, Handler::return_to_user as usize);
            vector.push(thread);
        }

        let inner = ModelProcess {
            id: pid, address_space, thread: vector, parent: None, children: Vec::new(), allocator
        };

        Self(inner)
    }

    fn new_kernel() -> Self {
        let id = Self::new_pid();

        let address_space = AddressSpace::new_kernel();
        let mut thread = Vec::new();
        thread.push(Thread::empty());

        let inner = ModelProcess {
            id,
            address_space,
            thread,
            parent: None,
            children: Vec::new(),
            allocator: Allocator::new(0, 0).unwrap()
        };

        Self(inner)
    }

    fn fork(&mut self) -> Self {
        let id = Self::new_pid();

        let thread = self.0.thread.iter().map(|x| x.clone()).collect();

        let inner = ModelProcess {
            id,
            thread,
            parent: Some(self.0.id),
            address_space: self.0.address_space.clone(),
            children: Vec::new(),
            allocator: self.0.allocator.clone(),
        };

        Self(inner)
    }
}
