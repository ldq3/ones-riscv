use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("paniced at {}: {} {}", location.file(), location.line(), info.message());
    } else {
        println!("paniced: {}", info.message());
    }

    use ones::cpu::Lib;
    use crate::cpu;

    cpu::Handler::shutdown(true)
}
