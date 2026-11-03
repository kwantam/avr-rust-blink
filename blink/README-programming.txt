avrdude -p m328p -c usbasp-clone -P usb -e -U flash:w:target/avr-atmega328p/release/blink.elf -v -v -v
avrdude -p m328p -c usbasp-clone -P usb -U hfuse:r:-:h -U lfuse:r:-:h
avrdude -p m328p -c usbasp-clone -P usb -U hfuse:w:0xdf:m
