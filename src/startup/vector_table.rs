use super::{_estack, default_hanlder, reset_handler};

struct VectorTableBuilder<const N: usize> {
    addr: usize,
    stack: &'static usize,
    vectors: [VectorItem; N],
}

struct VectorItem {
    address: VectorAddr,
    name: &'static str,
    value: VectorValue,
}

enum VectorAddr {
    Addr(usize),
    /// VectorAddr::Range(min, max)
    Range(usize, usize),
}

enum VectorValue {
    Reserved,
    Fn(extern "C" fn()),
}

impl<const N: usize> VectorTableBuilder<N> {
    const fn build(self) -> VectorTable<N> {
        let mut result = (self.stack, [None; N]);

        let (mut index, mut last_addr) = (0, self.addr);
        while index < N {
            index += 1;
        }

        result
    }
}

type VectorTable<const N: usize> = (&'static usize, [Option<extern "C" fn()>; N]);

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
            value: VectorValue::Fn(reset_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0008),
            name: "NMI",
            value: VectorValue::Fn(default_hanlder),
        },
    ],
}
.build();

