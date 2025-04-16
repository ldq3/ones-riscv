use alloc::vec::Vec;
use ones::{ 
    memory::page::frame::Frame,
    peripheral::{ Block, virtio::{ Hal, VirtIOHeader }, instance::virtio_block::VirtIOBlk }
};

#[allow(unused)]
const VIRTIO0: usize = 0x10008000;

pub struct VirtIOBlock {
    virtio_blk: VirtIOBlk<'static, VirtioHal>,
}

impl Block for VirtIOBlock {
    fn read(&mut self, block_id: usize, buf: &mut [u8]) {
        self.virtio_blk.read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }

    fn write(&mut self, block_id: usize, buf: &[u8]) {
        self.virtio_blk.write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    pub fn new() -> Self {
        let virtio_blk = unsafe {
            VirtIOBlk::<VirtioHal>::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap()
        };
        Self {
            virtio_blk,
        }
    }
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let vector = Frame::new_contig(pages);
        let mut holder = HOLDER.lock();
        let base = vector[0].number;

        for frame in vector {
            holder.push(frame);
        }

        base
    }

    fn dma_dealloc(_pa: usize, _pages: usize) -> i32 {
        0
    }

    fn phys_to_virt(addr: usize) -> usize {
        addr
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        vaddr
    }
}

use spin::Mutex;
use alloc::sync::Arc;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref HANDLER: Arc<Mutex<dyn Block>> = {
        Arc::new(Mutex::new(VirtIOBlock::new()))
    };
}

lazy_static! {
    pub static ref HOLDER: Arc<Mutex<Vec<Frame>>> = {
        Arc::new(Mutex::new(Vec::new()))
    };
}
