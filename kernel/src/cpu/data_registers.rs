
use ones::cpu::{ DataReg as DG, ModelDataReg };

pub struct DataReg(ModelDataReg);

impl DG for DataReg {
    #[inline]
    fn empty() -> Self {
        Self(ModelDataReg([0; 32]))
    }

    #[inline]
    fn iid(&self) -> usize {
        self.0.0[17]
    }

    #[inline]
    fn iret(&self) -> usize {
        self.0.0[10]
    }

    #[inline]
    fn iret_set(&mut self, value: usize) {
        self.0.0[10] = value;
    }

    #[inline]
    fn iarg(&self) -> [usize; 3] {
        self.0.0[10..=12].try_into().unwrap()
    }

    fn sp_set(&mut self, value: usize) {
        self.0.0[2] = value
    }
}
