#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

#[no_mangle]
pub extern "C" fn main() {
    port::B5::set_output();

    loop {
        port::B5::set_high();

        ruduino::delay::delay_ms(250);

        port::B5::set_low();

        ruduino::delay::delay_ms(250);
    }
}
