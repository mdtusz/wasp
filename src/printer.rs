use gcode::parser::Line;
use utils::Point3;
use teensy3::serial::Serial;
use hardware::HardwareGpio;
use hardware::HardwareTime;
use motion::CartesianMotionPlanner;

static LINE_ENDING: u8 = '\n' as u8;

struct Printer<'a, H: 'a> {
    /// Buffer for Gcodes that must be executed in order
    /// Moves, temperatures, fans, etc
    gcode_buffer: [Option<Line>; 32],
    gcode_buffer_head: u8,
    gcode_buffer_tail: u8,

    /// Buffer for Gcodes that should be executed as fast as possible
    /// E-stop, informations, etc
    immediate_gcode_buffer: [Option<Line>; 32],
    immediate_gcode_buffer_head: u8,
    immediate_gcode_buffer_tail: u8,

    serial: Serial,

    serial_buffer: [u8; 256],
    serial_bytes: u8,

    motion: CartesianMotionPlanner,

    hardware: &'a H,
}

impl<'a, H: HardwareGpio + HardwareTime + Debug> Printer<'a, H> {
    fn new(hardware: H) -> Printer {
        Printer {
            gcode_buffer: [None; 32],
            gcode_buffer_head: 0,
            gcode_buffer_tail: 0,
            immediate_gcode_buffer: [None; 32],
            immediate_gcode_buffer_head: 0,
            immediate_gcode_buffer_tail: 0,
            serial: Serial {},
            serial_buffer: [0; 256],
            serial_bytes: 0,
            hardware: hardware,
        }
    }

    fn recive_serial(&mut self) {
        if let Ok(byte) = serial.try_read_byte() {
            if byte != LINE_ENDING {
                if self.serial_bytes < 256 {
                    self.serial_bytes += 1;
                    self.serial_buffer[self.serial_bytes] = byte;
                }
            } else {
                match core::str::from_utf8(&self.serial_buffer[0..self.serial_bytes]) {
                    Ok(chars) => {
                        let lexer = Tokenizer::new(src.chars());
                        let tokens = lexer.filter_map(|t| t.ok());
                        let parser = Parser::new(tokens);
                        for line in parser {
                            match line {
                                Ok(line) => {
                                    println!("Recived: {:?}", line);
                                    self.gcode_buffer_head += 1;
                                    self.gcode_buffer[self.gcode_buffer_head] = Some(Line);
                                }
                                Err(err) => println!("!! Could not parse line: {:?}", err),
                            }
                        }
                    }
                    Err(err) => println!("!! Could not make into chars: {:?}", err),
                }
                self.serial_bytes = 0;
            }
        }
    }

    fn update(&mut self) {
        self.recive_serial();
    }


    fn move_to(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {}
}
