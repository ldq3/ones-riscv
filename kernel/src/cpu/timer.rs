#[allow(unused)]
pub trait Timer {
    fn now() -> usize;

    fn init() {
        Self::set_next_trigger();
    }

    fn set_next_trigger();

    // fn check();
}

// pub static mut TICKS: usize = 0;

pub struct Handler;

impl Timer for Handler {
    fn now() -> usize {
        use riscv::register::time;
        time::read()
    }
    
    fn set_next_trigger() {
        let next_trigger = Self::now() + config::FREQ / config::TICKS_PER_SEC;
        sbi_rt::set_timer(next_trigger as _);
    }
}

use core::cmp::Ordering;
/**
expire
*/
struct Record {
    expire: usize,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.expire == other.expire
    }
}

impl Eq for Record {}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = -(self.expire as isize);
        let b = -(other.expire as isize);
        Some(a.cmp(&b))
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use lazy_static::*;
use spin::Mutex;
use alloc::collections::BinaryHeap;
lazy_static! {
    /**
    队列
    */
    static ref TIMER: Mutex<BinaryHeap<Record>> = Mutex::new(BinaryHeap::<Record>::new());
}

mod config {
    // const TIMEBASW: usize = 100_000;
    pub const FREQ: usize = 12_500_000;
    pub const TICKS_PER_SEC: usize = 100;
}