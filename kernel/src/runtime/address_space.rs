use alloc::{ format, vec };
use ones::{ 
    memory::{ page::{ frame::Frame, Dependence, Table as _ }, Address, Flag },
    runtime::address_space::{ config::INTERVENE_TEXT, AddressSpace as A, ModelAddressSpace },
};
use crate::memory::page;

pub struct AddressSpace(pub ModelAddressSpace<page::Table>);

impl A for AddressSpace {
    fn from_elf(elf: &[u8]) -> Self {
        extern "C" {
            fn ttext();
        }

        use ones::memory::Address;
        let itext = Address::number(ttext as usize);

        let inner = ModelAddressSpace::from_elf(elf, itext);

        Self(inner)
    }

    fn clone(&self) -> Self {
        let segement = self.0.segement.clone();
        
        let mut page_table = page::Table::new();

        for (segment, map) in &segement {
            if let page::Map::Fixed(_) = map {
                page_table.map_area(segment.range, segment.flag);
            } else {
                page_table.map_area(segment.range, segment.flag);
            }
        }

        let inner = ModelAddressSpace {
            entry: self.0.entry,
            end: self.0.end,
            segement,
            page_table,
        };

        Self(inner)
    }
    /**
    will be used only once.
    */
    #[inline]
    fn new_kernel() -> Self {
        // use ones::runtime::address_space::AddressSpace as _;
        use ones::memory::Address;
        use crate::{ runtime::address_space::AddressSpace, config::END };

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
        let segement = AddressSpace::kernel_segement(MMIO, text, read_only_data, data, static_data, frame_data);

        let mut page_table = page::Table::new();

        for (segment, map) in &segement {
            if let page::Map::Fixed(frame_num) = map {
                unsafe {
                    page_table.fixed_map_area(segment.range, *frame_num, segment.flag);
                }
            } else {
                let frame = Frame::new();
                unsafe {
                    page_table.fixed_map_area(segment.range, frame.number, segment.flag);
                }
                page_table.frame(frame);
            }
        }

        let frame_nubmer = Address::number(ttext as usize);
        unsafe{ page_table.fixed_map(INTERVENE_TEXT, frame_nubmer, Flag::X | Flag::R) };

        let inner = ModelAddressSpace {
            entry: 0,
            end: 0,
            segement,
            page_table,
        };

        Self(inner)
    }

    fn new_stack(&mut self, tid: usize) -> usize {
        let (start, end, flag) = self.stack(tid);
        self.0.page_table.map_area((start, end), flag);

        Address::address(end + 1) - 1
    }

    fn new_intervene(&mut self, tid: usize) -> usize {
        let (page_number, flag) = Self::intervene_data(tid);
        self.0.page_table.map(page_number, flag);
        let (frame_number, _) = self.0.page_table.get(page_number);

        frame_number
    }

    #[inline]
    fn stack(&self, tid: usize) -> (usize, usize, Flag) {
        let bottom = self.0.end + (tid + 1) * (1 + config::STACK_SIZE);

        (bottom - config::STACK_SIZE + 1, bottom, Flag::W | Flag::R | Flag::U)
    }
}

use ones::info_module;

#[inline]
fn info<M>(msg: impl IntoIterator<Item = M>) 
    where M: AsRef<str>,
{
    info_module("address space", msg);
}

mod config {
    /// 单位：页
    pub const STACK_SIZE: usize = 2;
}