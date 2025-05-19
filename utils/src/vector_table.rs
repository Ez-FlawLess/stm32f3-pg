pub struct VectorTableBuilder<const N: usize> {
    pub addr: usize,
    pub stack: &'static usize,
    pub vectors: [VectorItem; N],
}

pub struct VectorItem {
    pub address: VectorAddr,
    pub name: &'static str,
    pub value: VectorValue,
}

pub enum VectorAddr {
    Addr(usize),
    /// VectorAddr::Range(min, max)
    Range(usize, usize),
}

pub enum VectorValue {
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
    pub const fn build<const M: usize>(self) -> VectorTable<M> {
        if M < N {
            panic!("VectorTable can't have less items than VectorTableBuilder");
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

pub type VectorTable<const N: usize> = (&'static usize, [Option<extern "C" fn()>; N]);

#[cfg(test)]
mod tests {
    use super::*;

    static mut STACK: usize = 0;
    static mut SHARED_NUM: i32 = 0;

    extern "C" fn create_handler() {
        unsafe {
            SHARED_NUM += 1;
        }
    }

    #[test]
    #[allow(static_mut_refs)]
    fn test_vector_table_builder() {
        let vector_table_builder = VectorTableBuilder::<7> {
            addr: 0,
            stack: unsafe { &STACK },
            vectors: [
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0008),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_0004),
                    name: "1",
                    value: VectorValue::Fn(create_handler),
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0010),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_0008),
                    name: "2",
                    value: VectorValue::Fn(create_handler),
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0018),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_000C),
                    name: "3",
                    value: VectorValue::Reserved,
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0020),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_0010),
                    name: "4",
                    value: VectorValue::Fn(create_handler),
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Range(0x0000_0028, 0x0000_0038),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Range(0x0000_0014, 0x0000_001C),
                    name: "5",
                    value: VectorValue::Reserved,
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0040),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_0020),
                    name: "6",
                    value: VectorValue::Fn(create_handler),
                },
                VectorItem {
                    #[cfg(target_pointer_width = "64")]
                    address: VectorAddr::Addr(0x0000_0048),
                    #[cfg(target_pointer_width = "32")]
                    address: VectorAddr::Addr(0x0000_0024),
                    name: "7",
                    value: VectorValue::Fn(create_handler),
                },
            ],
        };

        let expected_vector_table: VectorTable<9> = (
            unsafe { &STACK },
            [
                Some(create_handler), // 0x0000_0004
                Some(create_handler), // 0x0000_0008
                None,                 // 0x0000_000C
                Some(create_handler), // 0x0000_0010
                None,                 // 0x0000_0014
                None,                 // 0x0000_0018
                None,                 // 0x0000_001C
                Some(create_handler), // 0x0000_0020
                Some(create_handler), // 0x0000_0024
            ],
        );
        let result_vector_table: VectorTable<9> = vector_table_builder.build();

        assert_eq!(expected_vector_table, result_vector_table);
    }
}
