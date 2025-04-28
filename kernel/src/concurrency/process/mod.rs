pub mod thread;

use alloc::vec::Vec;
use ones::{
    concurrency::process::{ thread::Thread as _, ModelProcess, Process as P  },
    runtime::address_space::AddressSpace as _, Allocator
};
use ones::concurrency::process::thread::context::Context;

use crate::runtime::address_space::AddressSpace;
use thread::Thread;

pub struct Process(pub ModelProcess<Thread, AddressSpace>);

impl P for Process {
    fn new(elf_data: &[u8]) -> Self {
        let pid = Self::new_pid(); 

        use ones::runtime::address_space::AddressSpace as _;
        use crate::runtime::address_space::AddressSpace;

        let address_space = AddressSpace::from_elf(elf_data);

        use ones::Allocator;
        let allocator = Allocator::new(1, 15).unwrap();
        let mut thread = Vec::new();
        thread.push(Thread::new(pid, 0));
        
        let inner = ModelProcess {
            id: pid,
            address_space,
            thread,
            parent: None,
            children: Vec::new(),
            allocator
        };

        Self(inner)
    }

    #[inline]
    fn id(&self) -> usize {
        self.0.id
    }

    // #[inline]
    // fn table(&mut self) -> usize {
    //     self.0.address_space.0.page_table.root()
    // }

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

    #[inline]
    fn get_context_mut(&mut self, tid: usize) -> &mut Context {
        &mut self.0.thread[tid].0.context
    }

    #[inline]
    fn get_context_ref(&self, tid: usize) -> &Context {
        &self.0.thread[tid].0.context
    }
}
