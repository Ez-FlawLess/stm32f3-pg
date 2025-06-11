#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use exti::Exti;
use game::Game;
use gpio::{OwnedPin, Pin, GPIOA, GPIOE};
use nvic::Nvic;
use rcc::RCC;
use rtt_target::debug_rtt_init_default;
use syscfg::SysCfg;
use utils::gpio::{ModeReg, OwnedModeReg, PinMode, PinPupdr, PupdrReg};
use utils::register::ConstRegister;

mod gpio;
mod my_critical_section;
mod rcc;
mod syscfg;
mod exti;
mod nvic;
mod timers;
mod game;
pub mod startup;

const CLOCK: usize = 8_000_000;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
extern "C" fn exti0_button_handler() {
    BUTTON_PRESSED.store(true, Ordering::SeqCst);

    // clear interrupt flag
    Exti::new().pr1().pr0().write(1);
}

fn main() -> ! {
    debug_rtt_init_default!();
    
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
    
    let mut game = Game::new(leds).unwrap();
   
    let button = B1UserButton::new(gpioa.p0());

    let mut nvic = Nvic::new();
    // enable interrupt #6
    nvic.iser0().irq6().write(1);
    
    let mut syscfg = SysCfg::new();
    // map PA0 to EXTI0
    syscfg.exti_cr1().exti_0().write(0);
    
    let mut exti = Exti::new();
    // Rising Edge
    exti.rtsr1().tr0().write(1);
    // un-mask interrupt
    exti.imr1().mr0().write(1);
    
    button.wait_for_press();
   
    game.start();
    
    loop {
        game.step();

        if button.pressed() {
            if !game.check_for_win() {
                button.wait_for_press();

                game.reset(); 
            }
        }
    }
}

struct B1UserButton<'a>(&'a mut Pin<1207959552, 0>);

impl<'a> B1UserButton<'a> {
    fn new(pin: &'a mut Pin<1207959552, 0>) -> Self {
        pin.set_mode(PinMode::Input);
        pin.set_pupdr(PinPupdr::PullDown);
        
        Self(pin)
    }
    
    fn pressed(&self) -> bool {
        BUTTON_PRESSED.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    }
    
    fn wait_for_press(&self) {
        while !self.pressed() {}
    }
}