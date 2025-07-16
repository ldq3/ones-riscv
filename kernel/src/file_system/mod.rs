use alloc::sync::Arc;
use ones::file_system::{ Lib as L, Inode };

pub struct Lib;

impl L for Lib {
    fn create(_name: &str) -> Result<Arc<Inode>, ()> {
        todo!()
    }
}