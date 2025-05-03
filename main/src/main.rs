#![no_std]
#![no_main]

use rtt_target::{debug_rprintln, debug_rtt_init};

mod my_critical_section;
pub mod startup;

fn main() -> ! {
    debug_rtt_init!();

    loop {
        debug_rprintln!("Hello, World!");
    }
}
