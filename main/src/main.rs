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

    // const GPIOE: usize = 0x4800_1000;

    // const MODE_OFFSET: usize = 0x00;

    // let gpioe_mode = (GPIOE + MODE_OFFSET) as *mut u32;

    // unsafe {
    //     let mask = 0x55_55_55_55;
    //     *gpioe_mode |= mask;
    // }

    // const ODR_OFFSET: usize = 0x14;
    // const GPIOE_ODR: usize = GPIOE + ODR_OFFSET;

    // const BSRR_OFFSET: usize = 0x18;
    // const GPIOE_BSRR: usize = GPIOE + BSRR_OFFSET;

    // let gpioe_odr = GPIOE_ODR as *mut u32;
    // let mut before = 0;
    // let mut after = 1;

    // unsafe {
    //     before = *gpioe_odr;
    //     *gpioe_odr = 0xFFFF;
    //     after = *gpioe_odr;
    // };

    // let gpioe_bsrr = GPIOE_BSRR as *mut u32;
    // unsafe {
    //     *gpioe_bsrr = 0x0F;
    //     *gpioe_odr = 0x0F;
    // };

    loop {}
}
