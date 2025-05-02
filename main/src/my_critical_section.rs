use core::arch::asm;

struct MyCriticalSection;

critical_section::set_impl!(MyCriticalSection);

unsafe impl critical_section::Impl for MyCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let primask: u32;

        unsafe {
            asm!("mrs {}, PRIMASK", out(reg) primask, options(nostack, nomem));
            asm!("cpsid i", options(nostack, nomem));
        }

        primask
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        unsafe {
            asm!("msr PRIMASK, {}", in(reg) restore_state, options(nostack, nomem));
        }
    }
}
