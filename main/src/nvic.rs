use utils::register::Register;

const NVIC_ADDR: usize = 0xE000_E000;

const fn iserx_offset(x: usize) -> usize {
    if x > 7 {
        panic!("x can only be from 0 to 7");
    }

    0x100 + (0x04 * x)
}

const ISER0: usize = NVIC_ADDR + iserx_offset(0);

pub struct Nvic {
    iser0: NvicIser<ISER0>,
}

impl Nvic {
    pub fn new() -> Self {
        Self {
            iser0: NvicIser { 
                irq6: Register,
            },
        }
    }

    pub fn iser0(&mut self) -> &mut NvicIser<ISER0> {
        &mut self.iser0
    }
}

pub struct NvicIser<const ADDR: usize> {
    irq6: Register<ADDR, 6, 0>,
}

impl<const ADDR: usize> NvicIser<ADDR> {
    pub fn irq6(&mut self) -> &mut Register<ADDR, 6, 0> {
        &mut self.irq6
    }
}