#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct DelayRegs {
    pub prescaler: u16,
    pub auto_reload: u16,
}

impl DelayRegs {
    pub const fn new(clock: usize, ms: usize) -> Self {
        
        let (ticks, false) = (clock / 1000).overflowing_mul(ms) else {
            panic!("clock mul ms overflowed");
        };

        let mut aar: usize = u16::MAX as usize + 1;
        let mut rem: usize = u16::MAX as usize;
        let mut result = Self{
            auto_reload: 0,
            prescaler: 0,
        };

        while aar >= 1 {
            let m_aar = aar;
            aar -= 1;

            let psc = ticks / aar;

            if psc < 1 || psc > u16::MAX as usize + 1 {
                continue;
            }

            let n_rem = ticks % aar;

            if n_rem == 0 {
               result.prescaler = (psc - 1) as u16;
               result.auto_reload = (aar - 1) as u16;

               return result;
            }

            if n_rem * m_aar < rem * aar {
               rem = n_rem; 

               result.prescaler = (psc - 1) as u16;
               result.auto_reload = (aar - 1) as u16;
            }
            
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

        assert_eq!(DelayRegs { auto_reload: 63_999, prescaler: 250}, delay);
       
    }
}