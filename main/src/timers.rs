use core::{sync::atomic::{AtomicBool, Ordering}, usize};

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
/// TIMx event generation register
const TIMX_EGR_OFFSET: usize = 0x14;

const ARR_ADDR: usize = TIM7+TIMX_ARR_OFFSET;
type ArrReg = Register<ARR_ADDR, 0, 16>;

const PSC_ADDR: usize = TIM7 + TIMX_PSC_OFFSET;
type PscReg = Register<PSC_ADDR, 0, 16>;

const SR_ADDR: usize = TIM7 + TIMX_SR_OFFSET;
type SrReg = Register<SR_ADDR, 0, 1>;

const CEN_ADDR: usize = TIM7 + TIMX_CR1_OFFSET;
type CenReg = Register<CEN_ADDR, 0, 1>;

const EGR_ADDR: usize = TIM7 + TIMX_EGR_OFFSET;
type UgReg = Register<EGR_ADDR, 0, 1>;

static TIM7_CREATED: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct Tim7Delay {
    arr_reg: ArrReg,
    psc_reg: PscReg,
    sr_reg: SrReg,
    cen_reg: CenReg,
    ug_reg: UgReg,
}

pub trait Delay {
    fn start(&mut self);
    fn resume(&mut self);
    fn stop(&mut self);
    fn wait(&mut self);
    fn poll(&mut self) -> DelayPoll;
}

impl Tim7Delay {
    pub fn new<const MS: usize>() -> Result<Self, ()> {
        TIM7_CREATED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| {
                ()
            })
            .map(|_| {
                let delay_regs = const {
                    DelayRegs::new(8_000_000, MS)
                };
                
                let mut result = Self {
                    arr_reg: Register,
                    psc_reg: Register,
                    sr_reg: Register,
                    cen_reg: Register,
                    ug_reg: Register,
                };
                
                result.arr_reg.write(delay_regs.auto_reload as usize);
                result.psc_reg.write(delay_regs.prescaler as usize);
                
                result
            })
    }

    pub fn new_with_regs(delay_regs: &DelayRegs) -> Result<Self, ()> {
        TIM7_CREATED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| {
                ()
            })
            .map(|_| {
                let mut result = Self {
                    arr_reg: Register,
                    psc_reg: Register,
                    sr_reg: Register,
                    cen_reg: Register,
                    ug_reg: Register,
                };
                
                result.arr_reg.write(delay_regs.auto_reload as usize);
                result.psc_reg.write(delay_regs.prescaler as usize);

                result.ug_reg.write(1);
                result.sr_reg.write(0);
                
                result
            })
    }
}

impl Drop for Tim7Delay {
    fn drop(&mut self) {
        self.stop();
        TIM7_CREATED.store(false, Ordering::Release); 
    }
}

impl Delay for Tim7Delay {
    
    fn start(&mut self) {
        self.cen_reg.write(1);
    }

    fn resume(&mut self) {
        self.cen_reg.write(1);
    }

    fn stop(&mut self) {
        self.cen_reg.write(0);
    }

    fn wait(&mut self) {
        while self.sr_reg.read() == 0 {}       

        self.sr_reg.write(0);
    }

    fn poll(&mut self) -> DelayPoll {
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
