/*!
file entry.asm under every architecture subfolder is needed. 初始化栈
*/

pub mod lang_items;
pub mod heap;
pub mod address_space;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
