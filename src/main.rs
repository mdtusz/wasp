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

use firmware::stepper;
use firmware::stepper::StepError;

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
    let mut old_step = now;

    // Create stepper motor
    let mut stepper_motor = stepper::StepperMotor::new(stepper::Limit { min: 0, max: 100 },
                                                   stepper::Direction::Forward,
                                                   5,
                                                   6,
                                                   5000);

    'main: loop {
        now = bindings::micros();

        if now - old >= 1_000_000 {
            if bindings::digitalRead(LED_PIN) == bindings::HIGH as u8 {
                bindings::digitalWrite(LED_PIN, bindings::LOW as u8);
            } else {
                bindings::digitalWrite(LED_PIN, bindings::HIGH as u8);
            }

            let p1 = Point3::new(1.0, 2.0, 3.0);
            let p2 = Point3::new(3.0, 4.0, 5.0);

            println!("p1: {:?}", p1);
            println!("p2: {:?}", p2);
            println!("Add points: {:?}", p1 + p2);

            old = now;
        }

        if now - old_step >= 10000 {
            //println!("Stepping Motor");
            match stepper_motor.step() {
                Ok(_) => {},
                Err(err) => {
                    match err {
                        StepError::Limit(dir) => {
                            stepper_motor.change_direction();
                            //println!("Changing direction");
                        }
                        _ => {
                            //println!("{:?}", err);
                        }
                    }
                }
            }
            //println!("Step: {:?}", stepper_motor.get_step());
            old_step = now;
        }

        stepper_motor.update();

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
