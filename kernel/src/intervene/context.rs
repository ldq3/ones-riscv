use ones::intervene::context::{ UserContext as C, ModelUserContext };

pub struct UserContext(ModelUserContext);

impl C for UserContext {
    fn init(&mut self, entry: usize, sp: usize) {
        use riscv::register::sstatus::{ self, SPP };

        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        self.0.x = [0; 32];
        self.0.status = sstatus.bits();
        self.0.pc = entry;

        self.sp_set(sp);
    }

    #[inline]
    fn pc_add(&mut self, value: usize) {
        self.0.pc += value;
    }

    #[inline]
    fn iid(&self) -> usize {
        self.0.x[17]
    }

    #[inline]
    fn iret(&self) -> usize {
        self.0.x[10]
    }

    #[inline]
    fn iret_set(&mut self, value: usize) {
        self.0.x[10] = value;
    }

    #[inline]
    fn iarg(&self) -> [usize; 3] {
        self.0.x[10..=12].try_into().unwrap()
    }

    // fn empty() -> Self {
    //     use crate::intervene;

    //     let dist = intervene::Handler::user_dist as usize;

    //     let user_context = ModelUser {
    //         x: [0; 32],
    //         status: 0,
    //         pc: 0,
    //     };

    //     let inner = ModelData {
    //         user_context,

    //         kernel_satp: 0,
    //         kernel_sp: 0,

    //         dist, 
    //     };

    //     Self(inner)
    // }

    fn sp_set(&mut self, value: usize) {
        self.0.x[2] = value
    }
}
