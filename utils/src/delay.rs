#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct DelayRegs {
    pub prescaler: u16,
    pub auto_reload: u16,
}

impl DelayRegs {
    #[cfg(any(test, target_pointer_width = "32"))]
    pub const fn new(clock: usize, ms: usize) -> Self {
        if ms == 0 {
            panic!("delay time can't be 0");
        }
        
        let (ticks, false) = (clock / 1000).overflowing_mul(ms) else {
            panic!("clock mul ms overflowed");
        };

        // auto-reload
        let mut best_aar: usize = u16::MAX as usize + 1;
        // remainder
        let mut best_rem: usize = u16::MAX as usize;

        let mut result = Self{
            auto_reload: 0,
            prescaler: 0,
        };

        let mut aar = best_aar;
        while aar >= 1 {
            
            // prescaler
            // between 1 and 65536
            let psc = ticks / aar;

            if psc < 1 || psc > u16::MAX as usize + 1 {
                aar -= 1;
                continue;
            }

            let rem = ticks % aar;

            if rem == 0 {
                return Self{
                    auto_reload: (aar - 1) as u16,
                    prescaler: (psc - 1) as u16,
                };
            }

            if rem * best_aar < best_rem * aar {
                best_aar = aar;
                best_rem = rem;

                result.prescaler = (psc - 1) as u16;
                result.auto_reload = (aar - 1) as u16;
            }

            aar -= 1;
        }
        
        if result.auto_reload == 0 && result.prescaler == 0 {
            panic!("no result was found");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_delay_regs() {
        let delay = const {DelayRegs::new(8_000_000, 1)};

        assert_eq!(DelayRegs { auto_reload: 7999, prescaler: 0}, delay);

        let delay = const {DelayRegs::new(8_000_000, 1_000)};

        assert_eq!(DelayRegs { auto_reload: 63_999, prescaler: 124 }, delay);

        let delay = const {DelayRegs::new(8_000_000, 2_000)};

        assert_eq!(DelayRegs { auto_reload: 63_999, prescaler: 249}, delay);
       
       let delay = const {DelayRegs::new(8_000_000, 2_515)};

       assert_eq!(DelayRegs { auto_reload: 62874, prescaler: 319}, delay);
    }
}