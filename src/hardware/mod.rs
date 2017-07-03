
/// Anything that can read and write GPIO pins in hardware
/// Pin: The type for a pin number
/// Mode: The type representing the mode of a pin
trait HardwareGpio<Pin, Mode> {
    fn pin_mode(&mut self, pin: Pin, mode: Mode);
    fn digital_write(&mut self, pin: Pin, val: bool);
    fn digital_read(&mut self, pin: Pin) -> Option<bool>;
}

trait HardwareUart {
    fn readable(&self) -> bool;
    fn read_byte(&self) -> Result<u8, &'static str>;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), ()>;
}

trait HardwareTime {
    fn delay(&self, micros: u32);
    fn now(&self) -> u32;
}