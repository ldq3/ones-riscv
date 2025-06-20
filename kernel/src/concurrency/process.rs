use ones::{
    concurrency::process::{ Dependence, Lib as L, Process },
    memory::{ page::{ Lib as _, Table as PageTable }, Address },
    runtime::address_space::AddressSpace
};
use crate::{ memory::page::Lib as PageLib };

pub struct Lib;

impl Dependence for Lib {
    #[inline]
    fn copy_data(table: &mut PageTable, range: (usize, usize), data: &[u8]) {
        PageLib::copy_data(table, range, data);
    }
}

impl L for Lib {
    fn new(parent: Option<usize>, address_space: AddressSpace) -> usize {
        extern "C" {
            fn itext();
        }

        let mut page_table = PageTable::new();
        for segement in &address_space.segement {
            PageLib::map_area(&mut page_table, segement.range, segement.flag);
        }

        let segement = AddressSpace::itext();
        let frame_number = Address::number(itext as usize);
        PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

        Process::new(parent, address_space, page_table)
    }

    fn new_kernel(address_space: AddressSpace) -> usize {
        extern "C" {
            fn itext();
        }

        let mut page_table = PageTable::new();
        for segement in &address_space.segement {
            PageLib::fixed_map_area(&mut page_table, segement.range, segement.range.0, segement.flag);
        }
        let segement = AddressSpace::itext();
        let frame_number = Address::number(itext as usize);
        PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

        Process::new(None, address_space, page_table)
    }
}

use ones::info_module;

#[allow(unused)]
fn info<M>(msg: impl IntoIterator<Item = M>)
    where M: AsRef<str>,
{
    info_module::<M>("process", msg);
}