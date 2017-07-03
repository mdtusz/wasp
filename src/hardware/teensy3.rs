use teensy3::util as teensy
use hardware::HardwareGpio;

struct HardwareTeensy3 {

}

impl HardwareGpio<u8, teensy::PinMode> for HardwareTeensy3 {
    fn pin_mode(&mut self, pin: u8, mode: teensy::PinMode) {
        teensy::pin_mode(pin, mode);
    }

    fn digital_write(&mut self, pin: u8, val: bool) {
        teensy::digital_write(pin, val);
    }

    fn digital_read(&mut self, pin: u8) -> Option<bool> {
        Ok(teensy::digital_read(pin))
    }
}

impl HardwareTime for HardwareTeensy3 {
    fn delay(&self, micros: u32) {
        teensy3::delay(micros / 1000);
    }

    fn now(&self) -> f32 {
        unsafe {
            teensy3::bindings::micros();
        }
    }
}