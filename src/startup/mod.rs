use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use crate::main;

mod vector_table;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    /// end of stack
    static _estack: usize;
    /// Start of .data section in FLASH (LMA)
    static _sidata: u8;
    /// Start of .data section in RAM (VMA)
    static _sdata: u8;
    /// End of .data section in RAM (VMA)
    static _edata: u8;
    /// Start of .bss section in RAM (VMA)
    static _sbss: u8;
    /// End of .bss section in RAM (VMA)
    static _ebss: u8;
}

#[unsafe(no_mangle)]
#[allow(invalid_reference_casting)]
pub extern "C" fn reset_handler() {
    unsafe {
        // disable interrupts globally
        asm!("CPSID i", options(nostack, nomem));

        let mut sidata_ptr = &_sidata as *const _ as *mut u8;
        let mut sdata_ptr = &_sdata as *const _ as *mut u8;
        let edata_ptr: *const u8 = &_edata;

        while (sdata_ptr as usize) < (edata_ptr as usize) {
            let byte = read_volatile(sidata_ptr);
            write_volatile(sdata_ptr, byte);

            sidata_ptr = sidata_ptr.add(1);
            sdata_ptr = sdata_ptr.add(1);
        }

        let mut sbss_ptr = &_sbss as *const _ as *mut u8;
        let ebss_ptr: *const u8 = &_ebss;

        while (sbss_ptr as usize) < (ebss_ptr as usize) {
            write_volatile(sbss_ptr, 0);

            sbss_ptr = sbss_ptr.add(1);
        }

        // enable interrupts globally
        asm!("CPSIE i", options(nostack, nomem));
    }

    main();
}

#[unsafe(no_mangle)]
pub extern "C" fn default_hanlder() {}
