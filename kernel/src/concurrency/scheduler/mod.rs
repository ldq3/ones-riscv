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

impl Mod<Scheduler> for Handler { }

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref HANDLER: Mutex<Option<Scheduler>> = Mutex::new(None);
}