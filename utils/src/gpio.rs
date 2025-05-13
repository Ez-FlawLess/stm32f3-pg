use crate::register::write_register;

#[macro_export]
macro_rules! gpio {
    (@pin $pin: literal, $addr: literal, {
        $($reg: ident: $offset: literal),+,
    }) => {
        $crate::utils_paste! {
            $(
                impl $crate::gpio::[<$reg:camel Reg>]<{$addr + $offset}, $pin> for Pin<$addr, $pin> {}
            )+
        }
    };
    (
        $(
            $name: ident at $addr: literal => {
                pins: [$($pin: literal),+],
                registers: $regs: tt,
            }
        ),+
    ) => {
        $crate::utils_paste! {
            pub struct Pin<const ADDR: usize, const PIN: usize>;

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

                $(
                    gpio!(@pin $pin, $addr, $regs);
                )+
           )+
        }
    };
}

#[repr(usize)]
pub enum PinMode {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
}

pub trait ModeReg<const ADDR: usize, const PIN: usize> {
    fn set_mode(&mut self, mode: PinMode) {
        write_register(ADDR as *mut usize, PIN * 2, 2, mode as usize);
    }
}

/// Output Data Register
#[repr(usize)]
pub enum PinOdr {
    Inactive = 0,
    Active = 0b1,
}

pub trait OdrReg<const ADDR: usize, const PIN: usize> {
    fn set_odr(&mut self, odr: PinOdr) {
        write_register(ADDR as *mut usize, PIN, 1, odr as usize);
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    gpio! {
        GPIOE at 0x4800_1000 => {
            pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            registers: {
                mode: 0x00,
                odr: 0x14,
            },
        }
    }
}
