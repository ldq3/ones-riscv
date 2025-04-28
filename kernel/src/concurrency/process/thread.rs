use ones::{ concurrency::process::thread::context::Context, concurrency::process::thread::{ ModelThread, Thread as T } };

pub struct Thread(pub ModelThread);

impl T for Thread {
    fn new(pid: usize, tid: usize) -> Self {
        let mut context = Context::empty();

        use crate::intervene;
        use ones::intervene::Lib as _;
        context.pc = intervene::Handler::return_to_user as usize;
        
        let inner = ModelThread {
            pid,
            tid,
            context,
        };

        Self(inner)
    }

    fn empty() -> Self { 
        let context = Context::empty();

        let inner = ModelThread {
            pid: 0,
            tid: 0,
            context,
        };

        Self(inner)
    }

    fn clone(&self) -> Self {
        todo!()
    }
}