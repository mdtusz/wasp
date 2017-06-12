extern crate gcode;

use gcode::parser::{Line, Command, CommandKind, Number};
use printer::Printer;

#[derive(Debug)]
struct Interpeter {
    printer: Printer,
    line_num: u32,
}

impl Interpeter {
    pub fn new(printer: Printer) {
        Interpeter {
            printer: printer,
            line_num: 0,
        }
    }

    pub fn run(&self, line: Line) {
        match line {
            Line::ProgramNumber => {}
            Line::Cmd(cmd) => {
                match cmd.kind {
                    CommandKind::G => self.g(cmd)
                    CommandKind::M => {}
                    CommandKind::T => {}
                }
            }
        }
    }

    fn g(&self, cmd: Command) {
        if let cmd.number = Number::Integer(n) {
            match n {
                0 | 1 => {
                    self.printer.move_to(cmd.args.x, cmd.args.y, cmd.args.z);
                }
                _ => {}
            }
        }
    }
}