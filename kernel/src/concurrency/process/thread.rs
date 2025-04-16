use ones::{ intervene::context::KernelContext, concurrency::process::thread::{ ModelThread, Thread as T } };

pub struct Thread(pub ModelThread);

impl T for Thread {
    fn new(pid: usize, tid: usize, sp: usize, ra: usize) -> Self {
        let kernel_context = KernelContext::new(sp, ra);
        
        let inner = ModelThread {
            pid,
            tid,
            kernel_context,
        };

        Self(inner)
    }

    fn empty() -> Self { 
        let kernel_context = KernelContext::empty();

        let inner = ModelThread {
            pid: 0,
            tid: 0,
            kernel_context,
        };

        Self(inner)
    }

    fn clone(&self) -> Self {
        todo!()
    }
}