/*！
virtio queue
*/

use ones::{ 
    peripheral::virtio::queue::{ Dependence, Queue as Q, ModelQueue, Descriptor, Flag },
    memory::page::frame::Frame
};

pub struct Queue(ModelQueue<16>);

impl Dependence for Queue {
    #[inline]
    fn size(&self) -> u16 {
        self.0.size
    }

    #[inline]
    fn set_size(&mut self, value: u16) {
        self.0.size = value;
    }

    #[inline]
    fn free(&self) -> u16 {
        self.0.free
    }

    #[inline]
    fn set_free(&mut self, value: u16) {
        self.0.free = value;
    }

    #[inline]
    fn descriptor(&self, index: u16) -> Descriptor {
        self.0.hardware.descriptor[index as usize]
    }

    #[inline]
    fn set_descriptor(&mut self, index: u16, value: Descriptor) {
        self.0.hardware.descriptor[index as usize] = value;
    }

    #[inline]
    fn available(&mut self, index: u16, data: u16) {
        self.0.hardware.available.data[index as usize] = data;
    }

    #[inline]
    fn available_head(&self) -> u16 {
        self.0.available_head
    }

    #[inline]
    fn inc_available_head(&mut self) {
        self.0.available_head += 1;
    }

    #[inline]
    fn used(&self, index: u16) -> (u32, u32) {
        self.0.hardware.used.data[index as usize]
    }

    #[inline]
    fn used_head(&self) -> u16 {
        self.0.hardware.used.index
    }

    #[inline]
    fn used_tail(&self) -> u16 {
        self.0.used_tail
    }

    #[inline]
    fn inc_used_tail(&mut self) {
        self.0.used_tail += 1;
    }
}

impl Q for Queue {
    #[inline]
    fn new(frame: Frame) -> Self {
        let inner = ModelQueue::new(frame);
        Self(inner)
    }

    #[inline]
    fn frame_number(&self) -> u32 {
        self.0.frame.number as u32
    }

    fn send(&mut self, rdata: &[&[u8]], wdata: &[&mut [u8]]) -> Result<u16, ()> {
        let size = self.size() + (rdata.len() + wdata.len()) as u16;
        if size > Self::capacity() {
            return Err(());
        }

        let head = self.free();
        let mut tail = 0;

        for data in rdata.iter() {
            tail = self.free();
            let mut descriptor = self.descriptor(tail);
            descriptor.physical_address = data.as_ptr() as u64;
            descriptor.length = data.len() as u32;
            descriptor.flag = Flag::NEXT;
            self.set_descriptor(tail, descriptor);

            self.set_free(descriptor.next);
        }

        for data in wdata.iter() {
            tail = self.free();
            let mut descriptor = self.descriptor(tail);
            descriptor.physical_address = data.as_ptr() as u64;
            descriptor.length = data.len() as u32;
            descriptor.flag = Flag::NEXT | Flag::WRITE;
            self.set_descriptor(tail, descriptor);

            self.set_free(descriptor.next);
        }

        // set last_elem.next = NULL
        let mut descriptor = self.descriptor(tail);
        let mut flags = descriptor.flag;
        flags.remove(Flag::NEXT);
        descriptor.flag = flags;
        self.set_descriptor(tail, descriptor);

        self.set_size(size);

        let index = self.available_head();

        self.available(index, head);

        self.inc_available_head();

        Ok(head)
    }
}
