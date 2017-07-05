#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate teensy3;
extern crate gcode;
extern crate firmware;

use teensy3::bindings;
use teensy3::serial::Serial;
use gcode::{Tokenizer, Parser};

use firmware::hardware::HardwareGpio;
use firmware::hardware::HardwareTime;
use firmware::hardware::teensy::HardwareTeensy3;

use firmware::stepper;

use firmware::utils::Point3;

const LINE_ENDING: u8 = 10;
const BUFFER_SIZE: usize = 256;
const LED_PIN: u8 = 13;

#[no_mangle]
pub unsafe extern "C" fn main() {

    bindings::pinMode(LED_PIN, bindings::OUTPUT as u8);

    let ser = Serial {};
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut i = 0;
    let mut now = 0;
    let mut old = now;

    let hardware = HardwareTeensy3::new();

    hardware.delay(5000000);

    // Create test stepper motor
    let mut stepper_motor = stepper::StepperMotor::new(4 * 32, 0.0, 35.0, 5, 6, &hardware);

    match stepper_motor.set_current_velocity(1200.0) {
        Ok(steps) => {
            println!("Set speed to {} mm/min, or {} microseconds per step",
                     stepper_motor.get_current_velocity(),
                     steps);
        }
        Err(_) => {
            println!("Set speed {} mm/min is too fast!",
                     stepper_motor.get_current_velocity());
        }
    }

    'main: loop {
        now = bindings::micros();

        if now - old >= 1_000_000 {
            if bindings::digitalRead(LED_PIN) == bindings::HIGH as u8 {
                bindings::digitalWrite(LED_PIN, bindings::LOW as u8);
            } else {
                bindings::digitalWrite(LED_PIN, bindings::HIGH as u8);
            }

            //println!("{:?}", stepper_motor);

            old = now;
        }

        match stepper_motor.update() {
            Ok(_) => (),
            Err(direction) => {
                println!("Went too far {:?}!", direction);
                stepper_motor.set_current_direction(!direction)
            }
        }

        match ser.try_read_byte() {
            Ok(msg) => {
                if msg == LINE_ENDING {
                    let src = match core::str::from_utf8(&buf[0..i]) {
                        Ok(src) => src,
                        Err(err) => {
                            println!("Error: {:?}", err);
                            i = 0;
                            continue 'main;
                        }
                    };
                    let lexer = Tokenizer::new(src.chars());
                    let tokens = lexer.filter_map(|t| t.ok());
                    let parser = Parser::new(tokens);
                    for line in parser {
                        let line = match line {
                            Ok(line) => line,
                            Err(err) => {
                                println!("Error: {:?}", err);
                                i = 0;
                                continue 'main;
                            }
                        };

                        println!("{:?}", line);
                    }
                    i = 0;
                } else {
                    if i < BUFFER_SIZE {
                        buf[i] = msg;
                        i += 1;
                    }
                }
            }
            Err(_) => {}
        }
    }
}
