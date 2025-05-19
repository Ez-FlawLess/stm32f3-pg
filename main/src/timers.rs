use core::usize;

use utils::{delay::DelayRegs, register::{ConstRegister, Register}};

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


pub fn delay_ms<const MS: usize>() {
    
    let delay_regs = const {
        DelayRegs::new(8_000_000, MS)
    };
   
    ArrReg::write(delay_regs.auto_reload as usize);
    PscReg::write(delay_regs.prescaler as usize);

    CenReg::write(1);

    while SrReg::read() == 0 {
        
    }
    
    SrReg::write(0);
}
