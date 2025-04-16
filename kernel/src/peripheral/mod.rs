pub mod console;
pub mod disk;
// pub mod queue;

use ones::peripheral::Lib;

#[allow(unused)]
pub struct Handler;

impl Lib for Handler {
    fn handle() {
        todo!()
        // let source = plic::Handler::claim(0, config::HART_S);
        // match source {
        //     5 => KEYBOARD_DEVICE.handle_irq(),
        //     6 => MOUSE_DEVICE.handle_irq(),
        //     8 => BLOCK_DEVICE.handle_irq(),
        //     10 => UART.handle_irq(),
        //     _ => panic!("unsupported IRQ {}", source),
        // }
        // plic.complete(0, config::HART_S, source);
    }
}

pub mod config {
    pub const HART_M: usize = 0;
    pub const HART_S: usize = 1;

    /// (interrupt, priority)
    pub const INTERRUPT: [(usize, u32); 4] = [
        (5, 1), // keyboard
        (6, 1), // mouse
        (8, 1), // block device
        (10, 1) // uart
    ];
}