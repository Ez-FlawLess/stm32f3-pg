
file target/thumbv7em-none-eabihf/debug/stm32f3-pg

target extended-remote :1337

monitor reset halt

break reset_handler
