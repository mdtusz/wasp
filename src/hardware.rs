
pub enum PinMode {
    Input,
    Output,
    InputPullup,
    InputPulldown,
    OutputOpenDrain
}

/// Anything that can read and write GPIO pins in hardware
/// Pin: The type for a pin number
/// Mode: The type representing the mode of a pin
pub trait HardwareGpio {
    fn pin_mode(&mut self, pin: u8, mode: PinMode);
    fn digital_write(&mut self, pin: u8, val: bool);
    fn digital_read(&mut self, pin: u8) -> Option<bool>;
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