use core::ptr::{read_volatile, write_volatile};
use paste::paste;

const GPIOE_ADDR: usize = 0x4800_1000;

const MODE_OFFSET: usize = 0x00;
const ODR_OFFSET: usize = 0x14;

macro_rules! gpio {
    (
        $(
            $name: ident at $addr: literal => [$($pin: literal),+]
        ),+
    ) => {
        paste! {
            $(
                pub struct $name {
                    $([<p $pin>]: Pin<$addr, $pin>,)+
                }

                impl $name {
                    pub fn new() -> Self {
                        Self {
                            $([<p $pin>]: Pin,)+
                        }
                    }

                    $(
                        pub fn [<p $pin>](&mut self) -> &mut Pin<$addr, $pin> {
                            &mut self.[<p $pin>]
                        }
                    )+
                }
            )+
        }
    };
}

gpio! {
    GPIOE at 0x4800_1000 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
}

pub struct Pin<const BASE_ADDR: usize, const PIN_NUM: usize>;

#[repr(usize)]
pub enum PinMode {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
}

/// Output Data Register
#[repr(usize)]
pub enum PinOdr {
    Inactive = 0,
    Active = 0b1,
}

impl<const BASE_ADDR: usize, const PIN_NUM: usize> Pin<BASE_ADDR, PIN_NUM> {
    pub fn set_mode(&mut self, mode: PinMode) {
        let mode = mode as usize;

        let index = PIN_NUM * 2;

        let mask = mode << index;

        let addr = Self::register_addr(MODE_OFFSET);

        unsafe {
            write_volatile(addr, read_volatile(addr) | mask);
        }
    }

    pub fn set_odr(&mut self, odr: PinOdr) {
        let odr = odr as usize;

        let mask = odr << PIN_NUM;

        let addr = Self::register_addr(ODR_OFFSET);

        unsafe {
            write_volatile(addr, read_volatile(addr) | mask);
        }
    }

    const fn register_addr(offset: usize) -> *mut usize {
        (BASE_ADDR + offset) as *mut usize
    }
}
