use ones::concurrency::thread::context::{ Lib as L, Context };

pub struct Lib;

impl L for Lib {
    #[inline]
    fn iid(context: &Context) -> usize {
        context.data_reg[17]
    }
    
    #[inline]
    fn iret(context: &Context) -> usize {
        context.data_reg[10]
    }

    #[inline]
    fn iret_set(context: &mut Context, value: usize) {
        context.data_reg[10] = value;
    }

    #[inline]
    fn iarg(context: &Context) -> [usize; 3] {
        context.data_reg[10..=12].try_into().unwrap()
    }

    fn sp_set(context: &mut Context, value: usize) {
        context.data_reg[2] = value
    }
} 