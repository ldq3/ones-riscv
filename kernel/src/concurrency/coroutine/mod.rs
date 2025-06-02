use ones::concurrency::coroutine::{
    context::Context,
    Lib as L,
    Dep,
};

// use crate::intervene;
// use ones::intervene::Lib as _;
// context.pc = intervene::Handler::return_to_user as usize;

pub struct Lib;

use core::arch::global_asm;
global_asm!(include_str!("switch.S"));
extern "C" {
    fn switch(current: *mut Context, next: *const Context);
}

impl Dep for Lib {
    #[inline]
    fn switch(current: *mut Context, next: *const Context) {
        unsafe{ switch(current, next) };
    }
}

impl L for Lib { }