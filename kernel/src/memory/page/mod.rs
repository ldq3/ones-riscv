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
use ones::memory::{
    page::{ entry::{ Lib as _, Entry }, frame::Frame, Dependence, Lib as L, Table },
    Flag,
};
use entry::EntryLib;

pub struct Lib;

impl Dependence for Lib {
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

    fn get_mut(table: &mut Table, page_num: usize) -> &mut Entry {
        let index = Self::index(page_num);
        let table = Self::as_table(table.root.number);

        let mut current_entry = &mut table[index[0]];
        let frame_number = EntryLib::frame_number(current_entry);
        let mut current_table = Self::as_table(frame_number);

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            let frame_number= EntryLib::frame_number(current_entry);
            current_table = Self::as_table(frame_number);
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];

        current_entry
    }
}

impl L for Lib {
    fn map(table: &mut Table, page_num: usize, page_flag: Flag) {
        let index = Self::index(page_num);
        let root = Self::as_table(table.root.number);

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !EntryLib::flag(current_entry).is_valid() {
            let frame = Frame::new();
            let frame_number = frame.number;
            *current_entry = EntryLib::new(frame_number, Flag::V);
            table.frame.push(frame);

            Self::as_table(frame_number)
        } else {
            let frame_number = EntryLib::frame_number(current_entry);
            Self::as_table(frame_number)
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = if !EntryLib::flag(current_entry).is_valid() {
                let frame = Frame::new();
                let frame_number = frame.number;
                *current_entry = EntryLib::new(frame_number, Flag::V);
                table.frame.push(frame);

                Self::as_table(frame_number)
            } else {
                let frame_number = EntryLib::frame_number(current_entry);
                Self::as_table(frame_number)
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        if !EntryLib::flag(current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = EntryLib::new(frame.number, Flag::V | page_flag);
            table.frame.push(frame);
        }
    }

    fn unmap(table: &mut Table, page_num: usize) {
        let entry = Self::get_mut(table, page_num);

        EntryLib::flag_set(entry, Flag::empty());
    }

    fn fixed_map(table: &mut Table, page_num: usize, frame_num: usize, page_flag: Flag) {
        let index = Self::index(page_num);
        let root = Self::as_table(table.root.number);

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !EntryLib::flag(current_entry).is_valid() {
            let frame = Frame::new();
            let frame_number = frame.number;
            *current_entry = EntryLib::new(frame_number, Flag::V);
            table.frame.push(frame);
            Self::as_table(frame_number)
        } else { 
            let frame_number = EntryLib::frame_number(current_entry);
            Self::as_table(frame_number)
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]]; 
            current_table = if !EntryLib::flag(current_entry).is_valid() {
                let frame = Frame::new();
                let frame_number = frame.number;
                *current_entry = EntryLib::new(frame_number, Flag::V);
                table.frame.push(frame);

                Self::as_table(frame_number)
            } else {
                let frame_number = EntryLib::frame_number(current_entry);

                Self::as_table(frame_number)
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        *current_entry = EntryLib::new(frame_num, page_flag | Flag::V);
    }
    
    fn get(table: &mut Table, page_num: usize) -> (usize, Flag) {
        let index = Self::index(page_num); let root = Self::as_table(table.root.number);

        let mut current_entry = &mut root[index[0]];
        let frame_number = EntryLib::frame_number(current_entry);
        let mut current_table = Self::as_table(frame_number);

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            let frame_number = EntryLib::frame_number(current_entry);
            current_table = Self::as_table(frame_number);
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        (EntryLib::frame_number(current_entry), EntryLib::flag(current_entry))
    }

    fn copy_data(table: &mut Table, range: (usize, usize), data: &[u8]) {
        use ones::memory::Address;
        let page_size = Address::address(1);

        let mut start: usize = 0;
        let mut current = range.0;
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + page_size)];
            let (frame_number, _) = Self::get(table, current);
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