#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;
extern crate gcode;

use teensy3::bindings;
use teensy3::serial::Serial;
use gcode::{Tokenizer, Parser};

const LINE_ENDING: u8 = 10;
const BUFFER_SIZE: usize = 256;
const LED_PIN: u8 = 13;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[no_mangle]
pub unsafe extern fn main() {

    bindings::pinMode(LED_PIN, bindings::OUTPUT as u8);

    let ser = Serial{};
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut i = 0;
    let mut now = 0;
    let mut old = now;
    let mut pos = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    loop {
        now = bindings::micros();

        if now - old >= 1_000_000 {
            if bindings::digitalRead(LED_PIN) == bindings::HIGH as u8 {
                bindings::digitalWrite(LED_PIN, bindings::LOW as u8);
            } else {
                bindings::digitalWrite(LED_PIN, bindings::HIGH as u8);
            }
            old = now;
        }

        match ser.try_read_byte() {
            Ok(msg) => {
                if msg == LINE_ENDING {
                    let src = match core::str::from_utf8(&buf[0..i]) {
                        Ok(src) => src,
                        Err(err) => {
                            println!("{:?}", err);
                            continue;
                        },
                    };
                    let lexer = Tokenizer::new(src.chars());
                    let tokens = lexer.filter_map(|t| t.ok());
                    let parser = Parser::new(tokens);
                    for line in parser {
                        let line = match line {
                            Ok(line) => line,
                            Err(err) => {
                                println!("{:?}", err);
                                continue;
                            },
                        };
                        
                        println!("{:?}", line);
                    }
                    // ser.write_bytes(&buf[0..i]);
                    // ser.write_bytes("\n\r".as_bytes());
                    i = 0;
                } else {
                    if i < BUFFER_SIZE {
                        buf[i] = msg;
                        i += 1;
                    }
                }
            },
            Err(_) => {}
        }
    }
}
