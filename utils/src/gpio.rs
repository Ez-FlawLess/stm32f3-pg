const ODR_OFFSET: u32 = 0x14;

pub struct GPIO {
    odr: &'static mut u32,
}

impl GPIO {
    /// Pass the address found at peripheral register boundry address table
    ///
    /// # Safety
    ///
    /// The base address has to correct and based on the doc
    pub const unsafe fn new(base_addr: u32) -> Self {
        Self {
            odr: {
                let addr = (base_addr + ODR_OFFSET) as *mut u32;
                unsafe { &mut *addr }
            },
        }
    }
}
