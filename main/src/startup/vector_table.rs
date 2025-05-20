use crate::exti0_button_handler;

use super::{_estack, default_handler};
use utils::vector_table::*;

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table")]
pub static VECTOR_TABLE: VectorTable<100> = VectorTableBuilder {
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
        VectorItem {
            address: VectorAddr::Addr(0x0000_000C),
            name: "HardFault",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0010),
            name: "MemManage",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0014),
            name: "BusFault",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0018),
            name: "UsageFault",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Range(0x0000_001C, 0x0000_0028),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_002C),
            name: "SVCall",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Range(0x0000_0030, 0x0000_0034),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0038),
            name: "PendSV",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_003C),
            name: "SysTick",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0040),
            name: "WWDG",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0044),
            name: "PVD",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0048),
            name: "TAMPER_STAMP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_004C),
            name: "RTC_WKUP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0050),
            name: "FLASH",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0054),
            name: "RCC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0058),
            name: "EXTI0",
            value: VectorValue::Fn(exti0_button_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_005C),
            name: "EXTI1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0060),
            name: "EXTI2_TS",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0064),
            name: "EXTI3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0068),
            name: "EXTI4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_006C),
            name: "DMA1_Channel1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0070),
            name: "DMA1_Channel2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0074),
            name: "DMA1_Channel3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0078),
            name: "DMA1_Channel4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_007C),
            name: "DMA1_Channel5",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0080),
            name: "DMA1_Channel6",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0084),
            name: "DMA1_Channel7",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0088),
            name: "ADC1_2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_008C),
            name: "USB_HP/CAN_TX",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0090),
            name: "USB_LP/CAN_RX0",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0094),
            name: "CAN_RX1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0098),
            name: "CAN_SCE",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_009C),
            name: "EXTI9_5",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00A0),
            name: "TIM1_BRK/TIM15",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00A4),
            name: "TIM1_UP/TIM16",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00A8),
            name: "TIM1_TRG_COM/TIM17",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00AC),
            name: "TIM1_CC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00B0),
            name: "TIM2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00B4),
            name: "TIM3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00B8),
            name: "TIM4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00BC),
            name: "I2C1_EV",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00C0),
            name: "I2C1_ER",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00C4),
            name: "I2C2_EV",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00C8),
            name: "I2C2_ER",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00CC),
            name: "SPI1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00D0),
            name: "SPI2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00D4),
            name: "USART1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00D8),
            name: "USART2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00DC),
            name: "USART3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00E0),
            name: "EXTI15_10",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00E4),
            name: "RTC_Alarm",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00E8),
            name: "USBWakeUp",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00EC),
            name: "TIM8_BRK",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00F0),
            name: "TIM8_UP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00F4),
            name: "TIM8_TRG_COM",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00F8),
            name: "TIM8_CC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_00FC),
            name: "ADC3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0100),
            name: "FMC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0104),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0108),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_010C),
            name: "SPI3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0110),
            name: "UART4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0114),
            name: "UART5",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0118),
            name: "TIM6_DAC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_011C),
            name: "TIM7",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0120),
            name: "DMA2_Channel1",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0124),
            name: "DMA2_Channel2",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0128),
            name: "DMA2_Channel3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_012C),
            name: "DMA2_Channel4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0130),
            name: "DMA2_Channel5",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0134),
            name: "ADC4",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0138),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_013C),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0140),
            name: "COMP1_2_3",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0144),
            name: "COMP4_5_6",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0148),
            name: "COMP7",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_014C),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0150),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0154),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0158),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_015C),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0160),
            name: "I2C3_EV",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0164),
            name: "I2C3_ER",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0168),
            name: "USB_HP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_016C),
            name: "USB_LP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0170),
            name: "USB_WakeUp_RMP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0174),
            name: "TIM20_BRK",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0178),
            name: "TIM20_UP",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_017C),
            name: "TIM20_TRG_COM",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0180),
            name: "TIM20_CC",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0184),
            name: "FPU",
            value: VectorValue::Fn(default_handler),
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0188),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_018C),
            name: "",
            value: VectorValue::Reserved,
        },
        VectorItem {
            address: VectorAddr::Addr(0x0000_0190),
            name: "SPI4",
            value: VectorValue::Fn(default_handler),
        },
    ],
}
.build();
