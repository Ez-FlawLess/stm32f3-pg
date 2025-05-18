#![no_std]
#![no_main]

use gpio::{OwnedPin, GPIOA, GPIOE};
use rcc::RCC;
use rtt_target::debug_rtt_init_default;
use timers::delay;
use utils::gpio::{IdrReg, ModeReg, OwnedModeReg, OwnedOdrReg, PinIdr, PinMode, PinOdr, PinPupdr, PupdrReg};

mod gpio;
mod my_critical_section;
mod rcc;
mod timers;
pub mod startup;

fn main() -> ! {
    debug_rtt_init_default!();

    let mut rcc = RCC::new().unwrap();
    rcc.ahbenr().gpioa_en().enable_clock();
    rcc.ahbenr().gpioe_en().enable_clock();
    rcc.apb1().tim7_en().enable_clock();
   
    let mut gpioa= GPIOA::new();
    let mut gpioe = GPIOE::new();

    let mut leds: [OwnedPin; 8] = [
        gpioe.p9().to_owned(),
        gpioe.p10().to_owned(),
        gpioe.p11().to_owned(),
        gpioe.p12().to_owned(),
        gpioe.p13().to_owned(),
        gpioe.p14().to_owned(),
        gpioe.p15().to_owned(),
        gpioe.p8().to_owned(),
    ];

    for led in leds.iter_mut() {
        led.set_mode(PinMode::Output);
    }

    let button = gpioa.p0();
    button.set_mode(PinMode::Input);
    button.set_pupdr(PinPupdr::PullDown);

    let mut leds = leds.into_iter().cycle();

    let mut current_led = leds.next().unwrap();

    current_led.set_odr(PinOdr::Active);

    while let PinIdr::Inactive = button.get_idr() {}
    
    loop {
        delay();

        current_led.set_odr(PinOdr::Inactive);
        current_led = leds.next().unwrap();
        current_led.set_odr(PinOdr::Active);
   }
}
