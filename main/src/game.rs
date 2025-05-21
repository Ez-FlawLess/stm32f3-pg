use core::{array::IntoIter, iter::Cycle};

use utils::{delay::DelayRegs, gpio::{OwnedOdrReg, PinOdr}};

use crate::{gpio::OwnedPin, timers::{Delay, DelayPoll, Tim7Delay}, CLOCK};

static DELAYS: [DelayRegs; 8] = const {[
    DelayRegs::new(CLOCK, 1_000), // LEVES[0]
    DelayRegs::new(CLOCK, 750),   // LEVES[1]
    DelayRegs::new(CLOCK, 500),   // LEVES[2]
    DelayRegs::new(CLOCK, 400),   // LEVES[3]
    DelayRegs::new(CLOCK, 300),   // LEVES[4]
    DelayRegs::new(CLOCK, 200),   // LEVES[5]
    DelayRegs::new(CLOCK, 100),   // LEVES[6]
    DelayRegs::new(CLOCK, 50),   // LEVES[7]
]};

pub struct Game<const N: usize> {
    leds: [OwnedPin; N],
    indexes: Cycle<IntoIter<usize, N>>,
    current_index: usize,
    delay: Option<Tim7Delay>,
    level: usize,
}

impl<const N: usize> Game<N> {
    pub fn new(mut leds: [OwnedPin; N]) -> Result<Self, ()> {

        let mut indexes = 
            core::array::from_fn(|i| i).into_iter().cycle();

        leds[indexes.next().unwrap()].set_odr(PinOdr::Active);
    
        Ok(Self {
            leds,
            indexes,
            current_index: 0,
            delay: Some(Tim7Delay::new_with_regs(&DELAYS[0])?),
            level: 1,
        })
    }

    pub fn start(&mut self) {
        self.delay.as_mut().unwrap().start();
    }

    pub fn step(&mut self) {
        if let DelayPoll::Done = self.delay.as_mut().unwrap().poll() {
            self.next_led();
        }
    }

    pub fn check_for_win(&mut self) -> bool {
        self.delay.as_mut().unwrap().stop();
        if self.current_index == 0 {
            self.next_level();
            true
        } else {
            self.lose();

            false
        }
    }

    pub fn reset(&mut self) {
        todo!()
    }

    fn lose(&mut self) {
        for led in self.leds.iter_mut() {
            led.set_odr(PinOdr::Active);
        }
    }

    fn next_level(&mut self) {

        self.new_delay(&DelayRegs::new(CLOCK, 100));
        self.delay.as_mut().unwrap().start();

        self.leds[0].set_odr(PinOdr::Inactive);

        let mut i = 0;
        while i < 3 {
            self.delay.as_mut().unwrap().wait();
            
            self.leds[0].set_odr(PinOdr::Active);
            
            self.delay.as_mut().unwrap().wait();
            
            self.leds[0].set_odr(PinOdr::Inactive);
 
            i += 1;
        }
           
        self.leds[0].set_odr(PinOdr::Active);

        self.level += 1;

        let Some(delay_regs) = DELAYS.get(self.level - 1) else {
            return self.win();
        };
        
        self.new_delay(delay_regs);

        self.indexes = 
            core::array::from_fn(|i| i).into_iter().cycle();
        self.current_index = self.indexes.next().unwrap();
        self.start();
    }

    fn win(&mut self) {
        self.delay = None;
        self.leds[self.current_index].set_odr(PinOdr::Inactive);
    }

    fn next_led(&mut self) {
        self.leds[self.current_index].set_odr(PinOdr::Inactive);

        self.current_index = self.indexes.next().unwrap();
        self.leds[self.current_index].set_odr(PinOdr::Active);
    }
   
    fn new_delay(&mut self, delay: &DelayRegs) {
        self.delay = None;
        self.delay = Some(Tim7Delay::new_with_regs(delay).unwrap());
    }
}