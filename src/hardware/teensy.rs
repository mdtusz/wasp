use teensy3;
use hardware::HardwareGpio;
use hardware::HardwareTime;

#[derive(Debug)]
pub struct HardwareTeensy3 {

}

impl HardwareTeensy3 {
    pub fn new() -> HardwareTeensy3 {
        HardwareTeensy3 {}
    }
}

impl HardwareGpio for HardwareTeensy3 {
    fn pin_mode(&self, pin: u8, mode: teensy3::util::PinMode) {
        teensy3::util::pin_mode(pin, mode);
    }

    fn digital_write(&self, pin: u8, val: bool) {
        teensy3::util::digital_write(pin, val);
    }

    fn digital_read(&self, pin: u8) -> Option<bool> {
        Some(teensy3::util::digital_read(pin))
    }
}

impl HardwareTime for HardwareTeensy3 {
    fn delay(&self, micros: u32) {
        teensy3::util::delay(micros / 1000);
    }

    fn now(&self) -> u32 {
        unsafe {
            teensy3::bindings::micros()
        }
    }
}