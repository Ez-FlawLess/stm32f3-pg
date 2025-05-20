use core::sync::atomic::{AtomicBool, Ordering};

use utils::register::ConstRegister;

const RCC_ADDR: usize = 0x4002_1000;

const AHBENR_OFFSET: usize = 0x14;
const AHBENR_ADDR: usize = RCC_ADDR + AHBENR_OFFSET;

const APB1_OFFSET: usize = 0x1C;
const APB1_ADDR: usize = RCC_ADDR + APB1_OFFSET;

const APB2_OFFSET: usize = 0x18;
const APB2_ADDR: usize = RCC_ADDR + APB2_OFFSET;

static RCC_CREATED: AtomicBool = AtomicBool::new(false);

/// Reset and Clock Control
pub struct RCC {
    /// AHB peripheral clock enable register (RCC_AHBENR)
    ahbenr: RccAhbenr<AHBENR_ADDR>,
    apb1: RccApb1<APB1_ADDR>,
    apb2: RccApb2<APB2_ADDR>,
}

impl RCC {
    pub fn new() -> Result<Self, ()> {
        match RCC_CREATED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(Self {
                ahbenr: RccAhbenr {
                    gpioa_en: RccReg,
                    gpiob_en: RccReg,
                    gpioe_en: RccReg,
                },
                apb1: RccApb1 { 
                    tim7_en: RccReg, 
                },
                apb2: RccApb2 {
                    sys_cfg_rst: RccReg,
                },
            }),
            Err(_) => Err(()),
        }
    }

    pub fn ahbenr(&mut self) -> &mut RccAhbenr<AHBENR_ADDR> {
        &mut self.ahbenr
    }

    pub fn apb1(&mut self) -> &mut RccApb1<APB1_ADDR> {
        &mut self.apb1
    }

    pub fn apb2(&mut self) -> &mut RccApb2<APB2_ADDR> {
        &mut self.apb2
    }
}

pub struct RccAhbenr<const Addr: usize> {
    gpioa_en: RccReg<Addr, 17>,
    gpiob_en: RccReg<Addr, 18>,
    gpioe_en: RccReg<Addr, 21>,
}

impl<const Addr: usize> RccAhbenr<Addr> {
    pub fn gpioa_en(&mut self) -> &mut RccReg<Addr, 17> {
        &mut self.gpioa_en
    }

    pub fn gpiob_en(&mut self) -> &mut RccReg<Addr, 18> {
        &mut self.gpiob_en
    }

    pub fn gpioe_en(&mut self) -> &mut RccReg<Addr, 21> {
        &mut self.gpioe_en
    }
}

pub struct RccApb1<const ADDR: usize> {
    tim7_en: RccReg<ADDR, 5>,
}

impl<const ADDR: usize> RccApb1<ADDR> {
    pub fn tim7_en(&mut self) -> &mut RccReg<ADDR, 5> {
        &mut self.tim7_en
    }
}

pub struct RccApb2<const ADDR: usize> {
    sys_cfg_rst: RccReg<ADDR, 0>,
}

impl<const ADDR: usize> RccApb2<ADDR> {
    pub fn sys_cfg_rst(&mut self) -> &mut RccReg<ADDR, 0> {
        &mut self.sys_cfg_rst
    }
}

pub struct RccReg<const ADDR: usize, const BIT: usize>;

impl<const ADDR: usize, const BIT: usize> ConstRegister<ADDR, BIT, 1> for RccReg<ADDR, BIT> {}

impl<const ADDR: usize, const BIT: usize> RccReg<ADDR, BIT> {
    pub fn enable_clock(&mut self) {
        self.write(1);
    }

    pub fn disable_clock(&mut self) {
        self.write(0);
    }
}
