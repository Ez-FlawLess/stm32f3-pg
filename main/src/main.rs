#![no_std]
#![no_main]

use gpio::{GPIOE, PinMode, PinOdr};
use rcc::RCC;
use rtt_target::debug_rtt_init_default;

mod gpio;
mod my_critical_section;
mod rcc;
pub mod startup;

fn main() -> ! {
    debug_rtt_init_default!();

    let mut rcc = RCC::new().unwrap();
    rcc.ahbenr().gpioe_en().enable_clock();

    let mut gpioe = GPIOE::new();

    gpioe.p10().set_mode(PinMode::Output);
    gpioe.p10().set_odr(PinOdr::Active);

    loop {}
}
