use utils::gpio;

gpio! {
    GPIOE at 0x4800_1000 => {
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        registers: {
            mode: 0x00,
            odr: 0x14,
        },
    }
}