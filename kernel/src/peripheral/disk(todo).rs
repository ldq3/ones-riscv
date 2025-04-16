use alloc::sync::Arc;
use spin::Mutex;
use ones::{
    memory::page::frame::Frame,
    peripheral::{ instance::virtio_block::VirioBlock, virtio::queue::Queue as _, Block }
};
use crate::peripheral::queue::Queue;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref HANDLER: Arc<Mutex<dyn Block>> = {
        let frame = Frame::new();
        let queue = Queue::new(frame);
        unsafe { Arc::new(Mutex::new(VirioBlock::new(0x10008000, queue))) }
    };
}