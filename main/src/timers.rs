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

pub struct Delay<const MS: usize> {
    arr_reg: ArrReg,
    psc_reg: PscReg,
    sr_reg: SrReg,
    cen_reg: CenReg,
}

impl<const MS: usize> Delay<MS> {
    pub fn new() -> Self {
        let delay_regs = const {
            DelayRegs::new(8_000_000, MS)
        };

        let mut result = Self {
            arr_reg: Register,
            psc_reg: Register,
            sr_reg: Register,
            cen_reg: Register,
        };
    
        result.arr_reg.write(delay_regs.auto_reload as usize);
        result.psc_reg.write(delay_regs.prescaler as usize);

        result
    }

    pub fn start(&mut self) {
        self.cen_reg.write(1);
    }

    pub fn stop(&mut self) {
        self.cen_reg.write(0);
    }

    pub fn wait(&mut self) {
        while self.sr_reg.read() == 0 {}       

        self.sr_reg.write(0);
    }

    pub fn poll(&mut self) -> DelayPoll {
        match self.sr_reg.read() {
            0 => {
                DelayPoll::Wait
            },
            1 => {
                self.sr_reg.write(0);

                DelayPoll::Done
            },
            _ => unreachable!(),
        }
    }
}

pub enum DelayPoll {
    Done,
    Wait,
}
