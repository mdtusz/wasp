use teensy3::util as teensy
use hardware::HardwareGpio;

struct HardwareTeensy3 {

}

impl HardwareGpio<u8, teensy::PinMode> {
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