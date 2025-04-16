/*!
# SV 39
## 虚拟地址
const ADDRESS_WIDTH: usize = 39;

const NUMBER_WIDTH: usize = 27;

const OFFSET_WIDTH: usize = 12;

## 页表项
const FRAME_NUMER_WIDTH; // 参见 frame

const RSW_WIDTH: usize = 2;

const FLAG_WIDTH: usize = 8;
*/

pub mod frame;
pub mod entry;

use core::slice::from_raw_parts_mut;

use alloc::vec::Vec;
pub use ones::memory::page::Map;
use ones::memory::{ page::{ entry::Entry as _, frame::Frame, Dependence, ModelTable, Table as T }, Flag };
use entry::TableEntry;

pub struct Table(ModelTable);

impl Dependence for Table {
    fn index(page_num: usize) -> Vec<usize> {
        let mut page_num = page_num;
        let mut index = [0usize; 3];

        index[2] = page_num & 0b111_111_111;
        for i in (0..2).rev() {        
            page_num >>= 9;
            index[i] = page_num & 0b111_111_111;
        }

        index.to_vec()
    }

    #[inline]
    fn conf() -> usize {
        3
    }

    #[inline]
    fn new_entry(frame_number: usize, flag: Flag) -> usize {
        TableEntry::new(frame_number, flag).bits()
    }

    #[inline]
    fn flag(entry: usize) -> Flag {
        let entry = TableEntry::from_bits(entry);
        entry.flag()
    }

    #[inline]
    fn frame_number(entry: usize) -> usize {
        let entry = TableEntry::from_bits(entry);
        entry.frame_number()
    }

    #[inline]
    fn set_flag(entry: &mut usize, flag: Flag) {
        let mut wrapper = TableEntry::from_bits(*entry);
        wrapper.set_flag(flag);
        *entry = wrapper.bits();
    }

    #[inline]
    fn root_table(&mut self) -> &'static mut [usize] {
        use ones::memory::Address;

        let address = Address::address(self.0.root.number);
        unsafe { from_raw_parts_mut(address as *mut usize, 512) }
    }

    #[inline]
    fn frame(&mut self, frame: ones::memory::page::frame::Frame) {
        self.0.frame.push(frame);
    }
}

impl T for Table {
    fn new() -> Self {
        let root = Frame::new();

        let inner = ModelTable {
            root,
            frame: Vec::new()
        };

        Self(inner)
    }

    fn copy_data(&mut self, range: (usize, usize), data: &[u8]) {
        use ones::memory::Address;
        let page_size = Address::address(1);

        let mut start: usize = 0;
        let mut current = range.0;
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + page_size)];
            let (frame_number, _) = self.get(current);
            let address = Address::address(frame_number);
            let target = unsafe{ from_raw_parts_mut(address as *mut u8, src.len()) };
            target.copy_from_slice(src);

            start += page_size;
            if start >= len {
                break;
            }
            current += 1;
        }
    }
}