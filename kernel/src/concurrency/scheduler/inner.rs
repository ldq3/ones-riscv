use alloc::{ vec::Vec, collections::VecDeque };
use ones::concurrency::{ 
    scheduler::inner::{ Scheduler as S, Dependence, ModelScheduler},
    process::Process as _,
};
use crate::concurrency::process::Process;

pub struct Scheduler(pub ModelScheduler<Process>);

use core::arch::global_asm;
global_asm!(include_str!("switch.S"));

extern "C" {
    fn switch(current: usize, next: usize);
}

impl Dependence for Scheduler {
    #[inline]
    fn switch(current: usize, next: usize) {
        unsafe {
            switch(current, next);
        }
    }
}

impl S for Scheduler {
    #[inline]
    fn new() -> Self {
        let mut process = Vec::new();
        process.push(Process::new_kernel());

        let inner = ModelScheduler {
            process,
            ready: VecDeque::new(),
            running: (0, 0)
        };

        Self(inner)
    }

    #[inline]
    fn switch_to_idle(&mut self) {
        let (current, next) = self.0.switch_to_idle();

        Self::switch(current, next);
    }

    #[inline]
    fn switch_to_ready(&mut self) {
        let (current, next) = self.0.switch_to_ready();

        Self::switch(current, next);
    }
}