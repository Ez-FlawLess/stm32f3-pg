use core::{
    ptr::{read_volatile, write_volatile},
    sync::atomic::{AtomicBool, Ordering},
};

const RCC_ADDR: usize = 0x4002_1000;

const AHBENR_OFFSET: usize = 0x14;
const AHBENR_ADDR: usize = RCC_ADDR + AHBENR_OFFSET;

static RCC_CREATED: AtomicBool = AtomicBool::new(false);

/// Reset and Clock Control
pub struct RCC {
    /// AHB peripheral clock enable register (RCC_AHBENR)
    ahbenr: RccAhbenr<AHBENR_ADDR>,
}

impl RCC {
    pub fn new() -> Result<Self, ()> {
        match RCC_CREATED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(Self {
                ahbenr: RccAhbenr {
                    gpioe_en: RccAhbenrReg,
                },
            }),
            Err(_) => Err(()),
        }
    }

    pub fn ahbenr(&mut self) -> &mut RccAhbenr<AHBENR_ADDR> {
        &mut self.ahbenr
    }
}

pub struct RccAhbenr<const Addr: usize> {
    gpioe_en: RccAhbenrReg<Addr, 21>,
}

impl<const Addr: usize> RccAhbenr<Addr> {
    pub fn gpioe_en(&mut self) -> &mut RccAhbenrReg<Addr, 21> {
        &mut self.gpioe_en
    }
}

pub struct RccAhbenrReg<const Addr: usize, const Bit: usize>;

impl<const Addr: usize, const Bit: usize> RccAhbenrReg<Addr, Bit> {
    pub fn enable_clock(&mut self) {
        let reg: *mut usize = Addr as _;

        let mask = 1 << Bit;

        unsafe {
            write_volatile(reg, read_volatile(reg) | mask);
        };
    }

    pub fn disable_clock() {
        todo!()
    }
}
