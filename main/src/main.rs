#![no_std]
#![no_main]

use gpio::{GPIOA, GPIOE};
use rcc::RCC;
use rtt_target::debug_rtt_init_default;
use utils::gpio::{IdrReg, ModeReg, OdrReg, PinMode, PinOdr, PinIdr, PinPupdr, PupdrReg};

mod gpio;
mod my_critical_section;
mod rcc;
pub mod startup;

fn main() -> ! {
    debug_rtt_init_default!();

    let mut rcc = RCC::new().unwrap();
    rcc.ahbenr().gpioa_en().enable_clock();
    rcc.ahbenr().gpioe_en().enable_clock();
   
    let mut gpioa= GPIOA::new();
    let mut gpioe = GPIOE::new();

    let mut ld3 = gpioe.p9();
    let mut ld4 = gpioe.p8();
    let mut ld5 = gpioe.p10();
    let mut ld6 = gpioe.p15();
    let mut ld7 = gpioe.p11();
    let mut ld8 = gpioe.p14();
    let mut ld9 = gpioe.p12();
    let mut ld10 = gpioe.p13();

    gpioa.p0().set_mode(PinMode::Input);
    gpioa.p0().set_pupdr(PinPupdr::PullDown);

    gpioe.p10().set_mode(PinMode::Output);

    gpioe.p15().set_mode(PinMode::Output);
    gpioe.p15().set_odr(PinOdr::Active);

    loop {
        match gpioa.p0().get_idr(){
            PinIdr::Inactive => {
                match gpioe.p10().get_odr() {
                    PinOdr::Inactive => {},
                    PinOdr::Active => {
                        gpioe.p10().set_odr(PinOdr::Inactive);
                    },
                }
            },
            PinIdr::Active => {
                match gpioe.p10().get_odr() {
                    PinOdr::Inactive => {
                        gpioe.p10().set_odr(PinOdr::Active);
                    },
                    PinOdr::Active => {},
                }
 
            },
        }
    }
}
