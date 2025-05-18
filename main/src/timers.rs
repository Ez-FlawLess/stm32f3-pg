use core::u16;

use utils::register::{ConstRegister, Register};

const TIM7: usize = 0x4000_1400;

/// TIMx control register 1
const TIMX_CR1_OFFSET: usize = 0x00;
/// TIMx prescaler offset
const TIMX_PSC_OFFSET: usize = 0x28;
/// TIMx auto-reload register offset
const TIMX_ARR_OFFSET: usize = 0x2C;
/// TIMx status register offset
const TIMX_SR_OFFSET: usize = 0x10;

const ARR_ADDR: usize = TIM7+TIMX_ARR_OFFSET;
type ArrReg = Register<ARR_ADDR, 0, 16>;

const PSC_ADDR: usize = TIM7 + TIMX_PSC_OFFSET;
type PscReg = Register<PSC_ADDR, 0, 16>;

const SR_ADDR: usize = TIM7 + TIMX_SR_OFFSET;
type SrReg = Register<SR_ADDR, 0, 1>;

const CEN_ADDR: usize = TIM7 + TIMX_CR1_OFFSET;
type CenReg = Register<CEN_ADDR, 0, 1>;

const fn calc_delay(ms: usize) -> (usize, usize) {
    let clock: f32 = 8_000_000_f32 / 1_000_f32;
    let ticks = ms as f32 * clock;

    let mut arr = u16::MAX as f32 + 1_f32;

    let mut psc = None::<f32>;
    while arr >= 1_f32 {
        
    }

    todo!()
}

pub fn delay() {
    let arr: usize = 64_515;
    let psc: usize = 247;
   
    ArrReg::write(arr);
    PscReg::write(psc);

    CenReg::write(1);

    while SrReg::read() == 0 {
        
    }
    
    SrReg::write(0);
}
