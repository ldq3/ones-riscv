use alloc::sync::Arc;
use ones::file_system::{ Main, Inode };

pub struct Handler;

impl Main for Handler {
    fn create(_name: &str) -> Option<Arc<Inode>> {
        todo!()
    }
}