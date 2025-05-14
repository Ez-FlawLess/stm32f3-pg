use core::ptr::{read_volatile, write_volatile};

pub struct Register<const ADDR: usize, const INDEX: usize, const SIZE: usize>;

impl<const ADDR: usize, const INDEX: usize, const SIZE: usize> Register<ADDR, INDEX, SIZE> {
    pub fn write(value: usize) {
        let mask: usize = const { 
            if SIZE == 0 {
                0
            } else if SIZE >= usize::BITS as usize {
                usize::MAX
            } else {
                (1 << SIZE) - 1
            }
        };

        let mask = mask << INDEX;
        let value = value << INDEX;

        // let value = value & mask;
        let addr = ADDR as *mut usize;
        
        let current_value = unsafe {
            read_volatile(addr)
        };

        let new_value = (current_value & !mask) | value;
        unsafe {
            write_volatile(addr, new_value);
        }
    }
}

pub fn write_register(addr: *mut usize, index: usize, size: usize, value: usize) {
    let mask = if size == 0 {
        0
    } else if size >= usize::BITS as usize {
        usize::MAX
    } else {
        (1 << size) - 1
    };

    let mask = mask << index;
    let value = value << index;

    let current_value = unsafe {
        read_volatile(addr)
    };

    let new_value = (current_value & !mask) | value;
    unsafe {
        write_volatile(addr, new_value);
    }
}

pub fn read_register(addr: *const usize, index: usize, size: usize) -> usize {
    let mask = if size == 0 {
        return 0;
    } else if size >= usize::BITS as usize {
        usize::MAX
    } else {
        (1 << size) - 1
    };

    let value = unsafe {
        read_volatile(addr)
    };

    (value >> index) & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_register() {
        let num = 0_usize;

        write_register(&num as *const usize as *mut usize, 3, 2, 0b11);

        assert_eq!(0b11000, num);
    }

    #[test]
    fn test_read_register() {
        let num = 0b10100_usize;

        let value = read_register(&num as *const usize, 2, 3);

        assert_eq!(0b101, value);
    }
}