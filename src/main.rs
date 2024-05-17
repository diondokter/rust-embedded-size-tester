#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

use rtt_target::{rprint, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let mut my_str = arrayvec::ArrayString::<1024>::new();

    writeln!(my_str, "Test!").unwrap();
    writeln!(my_str, "Here's a number: {}", magic() as i32).unwrap();

    let length = my_str.chars().count();

    writeln!(my_str, "The length before this was: {}", length).unwrap();

    let length = my_str.chars().count();

    writeln!(my_str, "And now: {}", length).unwrap();

    rprint!("{}", my_str);

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

/// Get a value unknown to the compiler
fn magic() -> u32 {
    cortex_m::peripheral::DWT::cycle_count()
}
