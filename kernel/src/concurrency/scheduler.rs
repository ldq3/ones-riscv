use core::ops::DerefMut;

use ones::concurrency::{ process::Process as _, scheduler::{ Dependence, Main, Model } };
use crate::concurrency::process::Process;

pub struct Handler;

impl Dependence<Process> for Handler {
    fn open_file(_name: &str, _flag: ones::file_system::Flag) -> Option<ones::file_system::file::File> {
        todo!()
    }

    fn get_ref() -> &'static Mutex<Option<Model<Process>>> {
        &HANDLER
    }
}

// use crate::exception::context::Context;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

use ones::concurrency::process::thread::context::Context;
extern "C" {
    fn switch(current: *mut Context, next: *const Context);
}

impl Main<Process> for Handler {
    fn new_process(elf: &[u8]) {
        let process = Process::new(elf);
        Self::access(|scheduler| {
            let pid = process.0.id;
            scheduler.process.insert(pid, process);
            scheduler.ready.push_back((pid, 0));
        })
    }

    fn fork() {
        let mut handler = HANDLER.lock();
        if let Some(scheduler) = handler.deref_mut() {
            let (pid, _) = scheduler.running;
            let process = &mut scheduler.process[pid];
            let child = process.fork();
            scheduler.process.insert(child.0.id, child);
            scheduler.ready.push_back((pid, 0));
        } else {
            panic!("Process scheduler had been initialized.");
        }
    }

    fn spawn(_entry: usize, _arg: usize) {
        todo!()
    }

    fn switch_to_ready() {
        let (idle, next) = Self::access(|scheduler| {
            let kernel = &mut scheduler.process[0];
            let idle = &mut kernel.0.thread[0].0.kernel_context;
            let idle = idle as *mut _;

            let (pid, tid) = scheduler.ready.pop_back().unwrap();
            scheduler.running = (pid, tid);
            let process = &scheduler.process[pid];
            let next = &process.0.thread[tid].0.kernel_context as *const _;

            (idle, next)
        });

        unsafe{ switch(idle, next); }
    }

    fn switch_to_idle() {
        Self::access(|scheduler| {
            let (pid, tid) = scheduler.running;

            let process = &mut scheduler.process[pid];
            let current = &mut process.0.thread[tid].0.kernel_context;
            let current = current as *mut _;

            let kernel = &scheduler.process[0];
            let idle = &kernel.0.thread[0].0.kernel_context;

            unsafe{ switch(current, idle); }
        });
    }
}

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref HANDLER: Mutex<Option<Model<Process>>> = Mutex::new(None);
}