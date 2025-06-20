/*!
# SV 39
## 虚拟地址
const ADDRESS_WIDTH: usize = 39;

const NUMBER_WIDTH: usize = 27;

const OFFSET_WIDTH: usize = 12;

## 物理地址
const ADDRESS_WIDTH: usize = 56;

const NUMBER_WIDTH: usize = 44;

const OFFSET_WIDTH: usize = 12; 

## 页表项
const FRAME_NUMER_WIDTH; // 参见 frame

const RSW_WIDTH: usize = 2;

const FLAG_WIDTH: usize = 8;
*/

pub mod entry;

use alloc::vec::Vec;
use ones::memory::{
    page::{ entry::{ Lib as _, Entry }, Dependence, Lib as L },
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

    #[inline]
    fn flag(entry: &Entry) -> Flag {
        EntryLib::flag(entry)
    }

    #[inline]
    fn new_entry(frame_num: usize, page_flag: Flag) -> Entry {
        EntryLib::new(frame_num, page_flag)
    }

    #[inline]
    fn frame_number(entry: &Entry) -> usize {
        EntryLib::frame_number(entry)
    }

    #[inline]
    fn set_flag(entry: &mut Entry, page_flag: Flag) {
        EntryLib::flag_set(entry, page_flag);
    }
}

impl L for Lib { }