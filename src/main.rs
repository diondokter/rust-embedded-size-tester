#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let mut my_str = arrayvec::ArrayString::<1024>::new();

    writeln!(my_str, "Test!").unwrap();

    cp.DWT.set_cycle_count(0);
    writeln!(
        my_str,
        "Here's a number: {}",
        (magic() as i32).saturating_add(10000000)
    )
    .unwrap();
    rprintln!("i32 fmt cycles: {}", cortex_m::peripheral::DWT::cycle_count());

    cp.DWT.set_cycle_count(0);
    let length = my_str.chars().count();
    rprintln!("str chars count cycles: {}", cortex_m::peripheral::DWT::cycle_count());

    writeln!(my_str, "The length before this was: {}", length).unwrap();

    let length = my_str.chars().count();

    writeln!(my_str, "And now: {}", length).unwrap();

    rprintln!("{}", my_str);

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
