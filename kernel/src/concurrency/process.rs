use alloc::{ vec, format };
use ones::{
    concurrency::process::{ Lib as L, Process },
    file_system::{ Flag, Main },
    memory::{ page::{ Lib as _, Table as PageTable }, Address },
    runtime::address_space::AddressSpace
};
use crate::{ file_system, memory::page::Lib as PageLib };

pub struct Lib;

impl L for Lib {
    fn new_kernel() {
        use ones::{
            memory::Address,
            runtime::address_space::AddressSpace,
        };
        use crate::config::END;

        extern "C" {
            fn stext();
            fn etext();

            fn srodata();
            fn erodata();

            fn sdata();
            fn edata();

            fn kernel_stack();
            // sbss
            fn ebss();

            fn ekernel();

            fn ttext();
        }

        let text = (Address::number(stext as usize), Address::number(etext as usize));
        let read_only_data = (Address::number(srodata as usize), Address::number(erodata as usize));
        let data = (Address::number(sdata as usize), Address::number(edata as usize));
        let static_data = (Address::number(kernel_stack as usize), Address::number(ebss as usize));
        let frame_data = (Address::number(ekernel as usize), Address::number(END));

        info(vec![
            format!("Segement text: {:x} - {:x}", text.0, text.1),
            format!("Segement read only data: {:x} - {:x}", read_only_data.0, read_only_data.1),
            format!("Segement data: {:x} - {:x}", data.0, data.1),
            format!("Segement static data: {:x} - {:x}", static_data.0, static_data.1),
            format!("Segement frame: {:x} - {:x}", frame_data.0, frame_data.1)
        ]);

        use crate::config::MMIO;
        let address_space = AddressSpace::new_kernel(0, MMIO, text, read_only_data, data, static_data, frame_data); // FIXME

        let mut page_table = PageTable::new();
        for segement in &address_space.segement {
            PageLib::fixed_map_area(&mut page_table, segement.range, segement.range.0, segement.flag);
        }
        let segement = AddressSpace::itext();
        let frame_number = Address::number(ttext as usize);
        PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

        Process::new(None, address_space, page_table);
    }

    fn init() -> usize {
        let file = file_system::Handler::open_file("init", Flag::R_W);
        if let Some(mut file) = file {
            let elf = file.read_all();
            extern "C" {
                fn ttext();
            }
            let (address_space, data_offset) = AddressSpace::from_elf(&elf);

            let mut page_table = PageTable::new();
            for i in 0..data_offset.len() { 
                let segement= address_space.segement[i];
                PageLib::map_area(&mut page_table, segement.range, segement.flag);

                PageLib::copy_data(&mut page_table, segement.range, &elf[data_offset[i].0..data_offset[i].1]);
            } 
            let segement = AddressSpace::itext();
            let frame_number = Address::number(ttext as usize);
            PageLib::fixed_map(&mut page_table, segement.range.0 , frame_number, segement.flag);

            Process::new(None, address_space, page_table)
        } else { panic!("Error when opening the init_process."); }
    }
}

use ones::info_module;
fn info<M>(msg: impl IntoIterator<Item = M>)
    where M: AsRef<str>,
{
    info_module::<M>("process", msg);
}