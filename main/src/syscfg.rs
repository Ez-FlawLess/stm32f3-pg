use utils::register::Register;

pub const SYSCFG_ADDR: usize = 0x4002_0000;

pub const EXTICR1_OFFSET: usize = 0x08;
pub const EXT1CR1_ADDR: usize = SYSCFG_ADDR + EXTICR1_OFFSET;

pub struct SysCfg {
    exti_cr1: SysCfgExtiCr1<EXT1CR1_ADDR>,
}

impl SysCfg {
    pub fn new() -> Self {
        Self {
            exti_cr1: SysCfgExtiCr1 { 
                exti_0: Register,
            }, 
        }
    }

    pub fn exti_cr1(&mut self) -> &mut SysCfgExtiCr1<EXT1CR1_ADDR> {
        &mut self.exti_cr1
    }
}

pub struct SysCfgExtiCr1<const ADDR: usize> {
    exti_0: Register<ADDR, 0, 4>,
}

impl<const ADDR: usize> SysCfgExtiCr1<ADDR> {
    pub fn exti_0(&mut self) -> &mut Register<ADDR, 0, 4> {
        &mut self.exti_0
    }
}