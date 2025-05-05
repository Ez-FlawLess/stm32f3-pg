#![no_std]
#![no_main]

use rtt_target::debug_rtt_init_default;

mod my_critical_section;
pub mod startup;

fn main() -> ! {
    debug_rtt_init_default!();

    loop {}
}
