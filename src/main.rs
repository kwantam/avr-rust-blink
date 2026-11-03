#![no_std]
#![no_main]

use ruduino::{Pin, Register};
use ruduino::cores::current::{port, UCSR0A, UCSR0B, UCSR0C, UBRR0, UDR0};

#[no_mangle]
pub extern "C" fn main() {
    port::B5::set_output();

    // set up the serial interface
    port::D0::set_input();
    port::D1::set_output();
    UCSR0A::set(UCSR0A::U2X0); // double data rate
    UCSR0B::write(UCSR0B::TXEN0 | UCSR0B::RXEN0); // enable TX and RX
    UCSR0C::write(UCSR0C::UCSZ00 | UCSR0C::UCSZ01); // 8n1
    UBRR0::write(51u8); // 38.4k, see page 199 of datasheet

    loop {
        port::B5::set_high();

        ruduino::delay::delay_ms(100);

        port::B5::set_low();

        for c in b"hello\r\n" {
            while UCSR0A::read() & 0x20 == 0 {}
            UDR0::write(*c);
        }

        ruduino::delay::delay_ms(100);
    }
}
