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
    (@owned_pin {
        $($reg: ident: $offset: literal),+,
    }) => {
        $crate::utils_paste! {
            $(
                impl $crate::gpio::[<Owned $reg:camel Reg>]<$offset> for OwnedPin {}
            )+
        }
    };
    (
        I/O: [$($name: ident at $addr: literal => [$($pin: literal),+],)+],
        registers: $regs: tt,
    ) => {
        $crate::utils_paste! {
            pub struct Pin<const ADDR: usize, const PIN: usize>;

            #[derive(Clone, Copy)]
            pub struct OwnedPin {
                addr: usize,
                pin: usize,
            }

            impl<const ADDR: usize, const PIN: usize> Pin<ADDR, PIN> {
                pub fn to_owned(&self) -> OwnedPin {
                    OwnedPin{
                        addr: ADDR,
                        pin: PIN
                    }
                }
            }

            impl $crate::gpio::OwnedPinT for OwnedPin {
                fn addr(&self) -> usize {
                    self.addr
                }

                fn pin(&self) -> usize {
                    self.pin
                }
            }

            gpio!(@owned_pin $regs);

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

pub trait OwnedPinT {
    fn addr(&self) -> usize;
    fn pin(&self) -> usize;
}

macro_rules! register_trait {
    (@write $name: ident, $size: literal) => {
        $crate::utils_paste! {
            fn [<set_ $name:snake>](&mut self, [<$name:snake>]: [<Pin $name:camel>]) {
                $crate::register::write_register(ADDR as *mut usize, PIN * $size, $size, [<$name:snake>] as usize);
            }
        }
    };
    (@read $name: ident, $size: literal) => {
        $crate::utils_paste! {
            fn [<get_ $name:snake>](&self) -> [<Pin $name:camel>] {
                let value = $crate::register::read_register(ADDR as *const usize, PIN * $size, $size);
                [<Pin $name:camel>]::try_from(value).expect("INTERNAL ERROR")
            }
        }
    };
    (@owned_write $name: ident, $size: literal) => {
        $crate::utils_paste! {
            fn [<set_ $name:snake>](&mut self, [<$name:snake>]: [<Pin $name:camel>]) {
                $crate::register::write_register((Self::addr(self) + OFFSET) as *mut usize, Self::pin(self) * $size, $size, [<$name:snake>] as usize);
            }
        }
    };
    (@owned_read $name: ident, $size: literal) => {
        $crate::utils_paste! {
            fn [<get_ $name:snake>](&self) -> [<Pin $name:camel>] {
                let value = $crate::register::read_register((Self::addr(self) + OFFSET) as *const usize, Self::pin(self) * $size, $size);
                [<Pin $name:camel>]::try_from(value).expect("INTERNAL ERROR")
            }
        }
    };
    (@ops rw, $name: ident, $size: literal) => {
        register_trait!(@write $name, $size);

        register_trait!(@read $name, $size);
    };
    (@ops r, $name: ident, $size: literal) => {
        register_trait!(@read $name, $size);
    };
    (@ops w, $name: ident, $size: literal) => {
        register_trait!(@write $name, $size);
    };   
    (@owned_ops rw, $name: ident, $size: literal) => {
        register_trait!(@owned_write $name, $size);

        register_trait!(@owned_read $name, $size);
    };
    (@owned_ops r, $name: ident, $size: literal) => {
        register_trait!(@owned_read $name, $size);
    };
    (@owned_ops w, $name: ident, $size: literal) => {
        register_trait!(@owned_write $name, $size);
    };
    ($name: ident, $ops: ident, $size: literal, {
        $($value_name: ident = $value: literal),+,
    }) => {
        $crate::utils_paste! {
            #[repr(usize)]
            pub enum [<Pin $name:camel>] {
                $($value_name = $value),+
            }

            impl core::convert::TryFrom<usize> for [<Pin $name:camel>] {
                type Error = ();

                fn try_from(value: usize) -> Result<Self, Self::Error> {
                    match value {
                        $($value => Ok(Self::$value_name),)+
                        _ => Err(()),
                    }
                }
            }

            pub trait [<$name:camel Reg>]<const ADDR: usize, const PIN: usize> {
                register_trait!(@ops $ops, $name, $size);
            }

            pub trait [<Owned $name:camel Reg>]<const OFFSET: usize>: OwnedPinT {
                register_trait!(@owned_ops $ops, $name, $size);
            }
        }
    };
}

register_trait!(mode, rw, 2, {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
});

// Input data register
register_trait!(idr, r, 1, {
    Inactive = 0b0,
    Active = 0b1,   
});

// Output data register
register_trait!(odr, rw, 1, {
    Inactive = 0b0,
    Active = 0b1,   
});

// pull-up pull-down register
register_trait!(pupdr, rw, 2, {
    NoPullUpPullDown = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
    Reserved = 0b11,
});

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    gpio! {
        I/O: [
            GPIOA at 0x4800_0000 => [0, 1, 2, 3],
            GPIOE at 0x4800_1000 => [0, 1, 2, 3, 4, 5, 6],
        ],
        registers: {
            mode: 0x00,
            odr: 0x14,
        },
    }
}
