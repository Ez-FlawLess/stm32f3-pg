#![no_std]
#![no_main]

use rtt_target::debug_rtt_init_default;

mod my_critical_section;
pub mod startup;

fn main() -> ! {
    debug_rtt_init_default!();

    const RCC_ADDR: usize = 0x4002_1000;
    const AHB_CE_OFFSET: usize = 0x014;

    let rcc_ahbenr = (RCC_ADDR + AHB_CE_OFFSET) as *mut u32;
    let mask = 1 << 21;

    unsafe {
        *rcc_ahbenr |= mask;
    }

    const GPIOE: usize = 0x4800_1000;

    const MODE_OFFSET: usize = 0x00;

    let gpioe_mode = (GPIOE + MODE_OFFSET) as *mut u32;

    unsafe {
        let mask = 0x55_55_55_55;
        *gpioe_mode |= mask;
    }

    const ODR_OFFSET: usize = 0x14;
    const GPIOE_ODR: usize = GPIOE + ODR_OFFSET;

    const BSRR_OFFSET: usize = 0x18;
    const GPIOE_BSRR: usize = GPIOE + BSRR_OFFSET;

    let gpioe_odr = GPIOE_ODR as *mut u32;
    let mut before = 0;
    let mut after = 1;

    unsafe {
        before = *gpioe_odr;
        *gpioe_odr = 0xFFFF;
        after = *gpioe_odr;
    };

    // let gpioe_bsrr = GPIOE_BSRR as *mut u32;
    // unsafe {
    //     *gpioe_bsrr = 0x0F;
    //     *gpioe_odr = 0x0F;
    // };

    loop {}
}
