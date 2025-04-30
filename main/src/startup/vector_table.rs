use super::{_estack, default_handler};
use utils::vector_table::*;

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table")]
pub static VECTOR_TABLE: VectorTable<2> = VectorTableBuilder {
    addr: 0,
    stack: unsafe { &_estack },
    vectors: [
        VectorItem {
            address: VectorAddr::Addr(0x0000_0004),
            name: "Reset",
            value: VectorValue::Fn(super::reset_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0008),
            name: "NMI",
            value: VectorValue::Fn(default_handler),
        },
    ],
}
.build();
