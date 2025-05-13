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
        I/O: [$($name: ident at $addr: literal => [$($pin: literal),+],)+],
        registers: $regs: tt,
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

macro_rules! register_trait {
    ($name: ident, $size: literal, {
        $($value_name: ident = $value: literal),+,
    }) => {
        $crate::utils_paste! {
            #[repr(usize)]
            pub enum [<Pin $name:camel>] {
                $($value_name = $value),+
            }

            pub trait [<$name:camel Reg>]<const ADDR: usize, const PIN: usize> {
                fn [<set_ $name:snake>](&mut self, [<$name:snake>]: [<Pin $name:camel>]) {
                    write_register(ADDR as *mut usize, PIN * $size, $size, [<$name:snake>] as usize);
                }
            }
        }
    };
}

register_trait!(mode, 2, {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
});

register_trait!(odr, 1, {
    Inactive = 0,
    Active = 0b1,   
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
