use teensy3;

pub mod teensy;

/// Anything that can read and write GPIO pins in hardware
/// Pin: The type for a pin number
/// Mode: The type representing the mode of a pin
pub trait HardwareGpio {
    fn pin_mode(&self, pin: u8, mode: teensy3::util::PinMode);
    fn digital_write(&self, pin: u8, val: bool);
    fn digital_read(&self, pin: u8) -> Option<bool>;
}

/// Anything that can provide a hardware Uart to talk to a host
pub trait HardwareUart {
    fn readable(&self) -> bool;
    fn read_byte(&self) -> Result<u8, &'static str>;
    fn write_bytes(&self, bytes: &[u8]) -> Result<(), ()>;
}

/// Anything that can provide the time since start and delay
/// All units are microseconds
pub trait HardwareTime {
    fn delay(&self, micros: u32);
    fn now(&self) -> u32;
}