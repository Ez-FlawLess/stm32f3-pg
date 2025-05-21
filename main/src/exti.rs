use utils::register::Register;

pub const EXTI_ADDR: usize = 0x4001_0400;

pub const IMR1_OFFSET: usize = 0x00;
pub const IMR1_ADDR: usize = EXTI_ADDR + IMR1_OFFSET;

pub const PR1_OFFSET: usize = 0x14;
pub const PR1_ADDR: usize = EXTI_ADDR + PR1_OFFSET;

pub const RTSR1_OFFSET: usize = 0x08;
pub const RTSR1_ADDR: usize = EXTI_ADDR + RTSR1_OFFSET;

pub const FTSR1_OFFSET: usize = 0x0C;
pub const FTSR1_ADDR: usize = EXTI_ADDR + FTSR1_OFFSET;

pub struct Exti {
    imr1: ExtiImr1<IMR1_ADDR>,
    pr1: ExtiPr1<PR1_ADDR>,
    rtsr1: ExtiRtsr1<RTSR1_ADDR>,
    ftsr1: ExtiFtsr1<FTSR1_ADDR>,
}

impl Exti {
    pub fn new() -> Self {
        Self {
            imr1: ExtiImr1 {
                mr0: Register,
            },
            pr1: ExtiPr1 { 
                pr0: Register,
            },
            rtsr1: ExtiRtsr1 {
                tr0: Register,
            },
            ftsr1: ExtiFtsr1 {
                tr0: Register,
            },
        }
    }

    pub fn imr1(&mut self) -> &mut ExtiImr1<IMR1_ADDR> {
        &mut self.imr1
    }

    pub fn pr1(&mut self) -> &mut ExtiPr1<PR1_ADDR> {
        &mut self.pr1
    }

    pub fn rtsr1(&mut self) -> &mut ExtiRtsr1<RTSR1_ADDR> {
        &mut self.rtsr1
    }

    pub fn ftsr1(&mut self) -> &mut ExtiFtsr1<FTSR1_ADDR> {
        &mut self.ftsr1
    }
}

pub struct ExtiImr1<const ADDR: usize> {
    mr0: Register<ADDR, 0, 1>,
}

impl<const ADDR: usize> ExtiImr1<ADDR> {
    pub fn mr0(&mut self) -> &mut Register<ADDR, 0, 1> {
        &mut self.mr0
    }
}

pub struct ExtiPr1<const ADDR: usize> {
    pr0: Register<ADDR, 0, 1>,
}

impl<const ADDR: usize> ExtiPr1<ADDR> {
    pub fn pr0(&mut self) -> &mut Register<ADDR, 0, 1> {
        &mut self.pr0
    }
}

pub struct ExtiRtsr1<const ADDR: usize> {
    tr0: Register<ADDR, 0, 1>,
}

impl<const ADDR: usize> ExtiRtsr1<ADDR> {
    pub fn tr0(&mut self) -> &mut Register<ADDR, 0, 1> {
        &mut self.tr0
    }
}

pub struct ExtiFtsr1<const ADDR: usize> {
    tr0: Register<ADDR, 0, 1>,
}

impl<const ADDR: usize> ExtiFtsr1<ADDR> {
    pub fn tr0(&mut self) -> &mut Register<ADDR, 0, 1> {
        &mut self.tr0
    }
}