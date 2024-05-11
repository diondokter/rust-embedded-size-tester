#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
