use core::usize;

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

struct VectorTableBuilderState<const N: usize> {
    result: VectorTable<N>,
    builder_index: usize,
    result_index: usize,
    last_addr: usize,
}

impl<const N: usize> VectorTableBuilderState<N> {
    const fn new<const M: usize>(builder: &VectorTableBuilder<M>) -> Self {
        Self {
            result: (builder.stack, [None; N]),
            builder_index: 0,
            result_index: 0,
            last_addr: builder.addr,
        }
    }

    const fn add_to_result(&mut self, addr: usize, value: &VectorValue) {
        if (addr - self.last_addr) != size_of::<usize>() {
            panic!("vector table addresses are not aligned by usize");
        }

        self.last_addr = addr;

        self.result.1[self.result_index] = match value {
            VectorValue::Reserved => None,
            VectorValue::Fn(v_fn) => Some(*v_fn),
        };
        self.result_index += 1;
    }
}

impl<const N: usize> VectorTableBuilder<N> {
    const fn build<const M: usize>(self) -> VectorTable<M> {
        if M < N {
            panic!("VectorTable can't have less items than VectortableBuiler");
        }

        let mut state = VectorTableBuilderState::<M>::new(&self);

        while state.builder_index < N {
            if state.result_index >= M {
                todo!()
            }
            let item = &self.vectors[state.builder_index];

            match item.address {
                VectorAddr::Addr(addr) => state.add_to_result(addr, &item.value),
                VectorAddr::Range(min, max) => {
                    let mut addr = min;

                    loop {
                        if addr > max {
                            panic!("max or min are not aligned");
                        }

                        state.add_to_result(addr, &item.value);

                        if addr == max {
                            break;
                        }

                        addr += size_of::<usize>();
                    }
                }
            }

            state.builder_index += 1;
        }

        state.result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_table_builder() {
        panic!("hi");
    }
}
