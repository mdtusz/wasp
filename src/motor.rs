
use core::ops::Not;
use hardware::peripherals::digital_io::DigitalOutput;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward = 1,
    Backward = -1,
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Direction {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

pub trait Motor {
    fn set_speed(&mut self, speed: f32);
    fn set_direction(&mut self, direction: Direction);
}

pub struct StepperMotor<'a> {
    step_pin: &'a mut DigitalOutput,
    dir_pin: &'a mut DigitalOutput,
}

impl<'a> StepperMotor<'a> {
    fn new(step_pin: &'a mut DigitalOutput, dir_pin: &'a mut DigitalOutput) -> StepperMotor<'a> {
        StepperMotor {
            step_pin: step_pin,
            dir_pin: dir_pin,
        }
    }
}

impl<'a> Motor for StepperMotor<'a> {
    fn set_speed(&mut self, speed: f32) {}
    fn set_direction(&mut self, direction: Direction) {}
}
