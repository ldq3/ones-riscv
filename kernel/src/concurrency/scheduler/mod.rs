pub mod inner;

use ones::concurrency::scheduler::{ Dependence, Mod };
use inner::Scheduler;

pub struct Handler;

impl Dependence<Scheduler> for Handler {
    fn open_file(_name: &str, _flag: ones::file_system::Flag) -> Option<ones::file_system::file::File> {
        todo!()
    }

    #[inline]
    fn get_ref() -> &'static Mutex<Option<Scheduler>> {
        &HANDLER
    }
}

// {
//     use ones::concurrency::process::thread::Thread;

//     let tid = 0; 

//     let usp = address_space.new_stack(tid);

//     use ones::intervene::Lib as _;
//     use crate::intervene::Handler;
//     let thread = Thread::new(pid, tid, ksp, Handler::return_to_user as usize);
//     vector.push(thread);
// }

use core::arch::global_asm;
global_asm!(include_str!("switch.S"));

extern "C" {
    fn switch(current: usize, next: usize);
}

impl Mod<Scheduler> for Handler {
    #[inline]
    fn switch(current: usize, next: usize) {
        unsafe{ switch(current, next) };
    }
}

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref HANDLER: Mutex<Option<Scheduler>> = Mutex::new(None);
}
