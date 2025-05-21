#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use exti::Exti;
use gpio::{OwnedPin, GPIOA, GPIOE};
use nvic::Nvic;
use rcc::RCC;
use rtt_target::debug_rtt_init_default;
use syscfg::SysCfg;
use timers::{Delay, DelayPoll};
use utils::{gpio::{ModeReg, OwnedModeReg, OwnedOdrReg, PinMode, PinOdr, PinPupdr, PupdrReg}};
use utils::register::ConstRegister;

mod gpio;
mod my_critical_section;
mod rcc;
mod syscfg;
mod exti;
mod nvic;
mod timers;
pub mod startup;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
extern "C" fn exti0_button_handler() {
    BUTTON_PRESSED.store(true, Ordering::SeqCst);

    Exti::new().pr1().pr0().write(1);
}

fn main() -> ! {
    debug_rtt_init_default!();

    // NVIC ISER0 address for enabling interrupts 0-31
    const NVIC_ISER0: *mut u32 = 0xE000E100 as *mut u32;
    // EXTI0_IRQn is typically 6 for STM32F3 series
    const EXTI0_IRQN: u32 = 6;

    
    let mut rcc = RCC::new().unwrap();
    rcc.ahbenr().gpioa_en().enable_clock();
    rcc.ahbenr().gpioe_en().enable_clock();
    rcc.apb1().tim7_en().enable_clock();
    rcc.apb2().sys_cfg_rst().enable_clock();
    
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
    
    let mut leds = leds.into_iter().cycle();
    
    let mut current_led = leds.next().unwrap();
    current_led.set_odr(PinOdr::Active);
    
    let button = gpioa.p0();
    button.set_mode(PinMode::Input);
    button.set_pupdr(PinPupdr::PullDown);
    
    let mut nvic = Nvic::new();
    nvic.iser0().irq6().write(1);
       
    let mut syscfg = SysCfg::new();
    syscfg.exti_cr1().exti_0().write(0);

    let mut exti = Exti::new();
    exti.rtsr1().tr0().write(1);
    exti.imr1().mr0().write(1);
    
    let mut delay = Delay::<50>::new();

    // while let PinIdr::Inactive = button.get_idr() {}
    while !BUTTON_PRESSED.load(Ordering::Relaxed) {}
    BUTTON_PRESSED.store(false, Ordering::SeqCst);
    
    delay.start();

    loop {
        if let DelayPoll::Done = delay.poll() {
            current_led.set_odr(PinOdr::Inactive);
            current_led = leds.next().unwrap();
            current_led.set_odr(PinOdr::Active);     
        }
        if BUTTON_PRESSED.load(Ordering::Relaxed) {
            delay.stop();
            BUTTON_PRESSED.store(false, Ordering::SeqCst);
            while !BUTTON_PRESSED.load(Ordering::Relaxed) {}
            delay.start();
            BUTTON_PRESSED.store(false, Ordering::SeqCst);
        }
    }
}
