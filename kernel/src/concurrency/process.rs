use ones::{
    concurrency::process::{ access, Lib as L, Process },
    memory::{ page::{ Lib as _, Table as PageTable }, Address },
    runtime::address_space::AddressSpace
};
use crate::{ memory::page::Lib as PageLib };

pub struct Lib;

impl L for Lib {
    fn new(parent: Option<usize>, address_space: AddressSpace) -> usize {
        extern "C" {
            fn ttext();
        }

        let mut page_table = PageTable::new();
        for segement in &address_space.segement {
            PageLib::map_area(&mut page_table, segement.range, segement.flag);
        }

        let segement = AddressSpace::itext();
        let frame_number = Address::number(ttext as usize);
        PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

        Process::new(parent, address_space, page_table)
    }

    fn new_kernel(address_space: AddressSpace) -> usize {
        extern "C" {
            fn ttext();
        }

        let mut page_table = PageTable::new();
        for segement in &address_space.segement {
            PageLib::fixed_map_area(&mut page_table, segement.range, segement.range.0, segement.flag);
        }
        let segement = AddressSpace::itext();
        let frame_number = Address::number(ttext as usize);
        PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

        Process::new(None, address_space, page_table)
    }

    fn from_elf(parent: Option<usize>, elf: &[u8]) -> usize {
        let (address_space, data_offset) = AddressSpace::from_elf(&elf);
        let pid = Self::new(parent, address_space);

        access(|manager| {
            let process = manager.process[pid].as_mut().unwrap();
            for i in 0..data_offset.len() { 
                let segement= process.address_space.segement[i];

                PageLib::copy_data(&mut process.page_table, segement.range, &elf[data_offset[i].0..data_offset[i].1]);
            }
        });
 
        pid
    }
}

use ones::info_module;

#[allow(unused)]
fn info<M>(msg: impl IntoIterator<Item = M>)
    where M: AsRef<str>,
{
    info_module::<M>("process", msg);
}