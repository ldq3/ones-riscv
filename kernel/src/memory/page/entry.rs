use ones::memory::{
    Flag,
    page::entry::{ Lib, Entry }
};

const FRAME_NUMBER_MASK: usize = 0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000;
const FLAG_MASK: usize = 0b11_111_111_111;

pub struct EntryLib;

impl Lib for EntryLib {
    fn new(frame_num: usize, page_flag: Flag) -> Entry {
        let frame_number_bits = frame_num << FRAME_NUMBER_MASK.trailing_zeros();
        let flag_bits = (page_flag.bits() as usize) << FLAG_MASK.trailing_zeros();

        Entry::from_bits(frame_number_bits | flag_bits)
    }

    fn frame_number(entry: &Entry) -> usize {
        (entry.bits() & FRAME_NUMBER_MASK) >> FRAME_NUMBER_MASK.trailing_zeros()
    }

    fn flag(entry: &Entry) -> Flag {
        let flag = (entry.bits() & FLAG_MASK) >> FLAG_MASK.trailing_zeros();

        Flag::from_bits(flag as u8).unwrap()
    }

    fn flag_set(entry: &mut Entry, page_flag: Flag) {
        let frame_number_bits = entry.bits() & FRAME_NUMBER_MASK;
        let flag_bits = (page_flag.bits() as usize) << FLAG_MASK.trailing_zeros();

        entry.bits_set(frame_number_bits | flag_bits); 
    }
}
