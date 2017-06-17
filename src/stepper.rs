use teensy3::bindings;
use motion::CartesianAxisMove;

/// The size of the buffer for StepperMotorController
/// Right now, just store the current move and the next one
const MOVE_BUFFER: usize = 2;

/// StepperMotorController
///
/// Controls a single stepper motor to follow a path of motion::CartesianAxisMove,
/// one at a time.
///
/// The unit of distance is one step of the stepper motor, defined as steps_per_millimeter
/// The unit of time is microseconds
/// The unit of velocity is microseconds per step
///
#[derive(Debug)]
pub struct StepperMotorController {

    /// The current position in steps
    current_position: i32,

    /// The current velocity in microseconds per step
    current_velocity: i32,

    /// The conversion between steps and millimeters
    steps_per_millimeter: i32,


    /// The StepperMotor to control
    stepper_motor: StepperMotor,
}

impl StepperMotorController {
    /// Create a new StepperMotorController from a StepperMotor, steps_per_millimeter, and
    /// ticks_per_second
    pub fn new(stepper_motor: StepperMotor,
               steps_per_millimeter: i32,
               ticks_per_second: i32)
               -> StepperMotorController {
        StepperMotorController {
            current_position: 0,
            current_velocity: 0,

            steps_per_millimeter: steps_per_millimeter,
            ticks_per_second: ticks_per_second,

            stepper_motor: stepper_motor,
        }
    }

    /// Get the current position converted to mm
    pub fn get_current_position(&self) -> f32 {
        self.current_position as f32 / self.steps_per_millimeter as f32
    }

    /// Get the current velocity converted to mm/sec
    pub fn get_current_velocity(&self) -> f32 {
        60000000.0/(self.current_velocity * self.steps_per_millimeter)
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
pub enum StepError {
    Limit(Direction),
    MidPulse,
    Unknown,
}

#[derive(Debug)]
pub struct Limit {
    pub max: i32,
    pub min: i32,
}


#[derive(Debug)]
pub struct StepperMotor {
    step: i32,
    step_limit: Limit,
    direction: Direction,
    last_step: u32,
    step_pin: u8,
    direction_pin: u8,
    pulse_length: u32,
    mid_pulse: bool,
}

impl StepperMotor {
    pub fn new(limit: Limit,
               dir: Direction,
               step_pin: u8,
               direction_pin: u8,
               pulse_length: u32)
               -> StepperMotor {

        unsafe {
            bindings::pinMode(step_pin, bindings::OUTPUT as u8);
            bindings::pinMode(direction_pin, bindings::OUTPUT as u8);
        }

        StepperMotor {
            step: 0,
            step_limit: limit,
            direction: dir,
            last_step: 0,
            step_pin: step_pin,
            direction_pin: direction_pin,
            pulse_length: pulse_length,
            mid_pulse: true,
        }
    }


    pub fn step(&mut self) -> Result<(), StepError> {
        if self.mid_pulse {
            return Result::Err(StepError::MidPulse);
        } else if self.direction == Direction::Forward && self.step >= self.step_limit.max {
            return Result::Err(StepError::Limit(Direction::Forward));
        } else if self.direction == Direction::Backward && self.step <= self.step_limit.min {
            return Result::Err(StepError::Limit(Direction::Backward));
        } else {
            unsafe {
                bindings::digitalWrite(self.step_pin, bindings::HIGH as u8);
                self.last_step = bindings::micros();
            }
            match &self.direction {
                &Direction::Forward => self.step += 1,
                &Direction::Backward => self.step -= 1,
            }
            self.mid_pulse = true;
            return Result::Ok(());
        }
    }

    pub unsafe fn update(&mut self) {
        if self.mid_pulse && (bindings::micros() - self.last_step >= self.pulse_length) {
            bindings::digitalWrite(self.step_pin, bindings::LOW as u8);
            self.mid_pulse = false;
        }
    }

    pub fn get_step(&self) -> i32 {
        self.step
    }

    pub fn change_direction(&mut self) {
        match &self.direction {
            &Direction::Forward => {
                unsafe {
                    bindings::digitalWrite(self.direction_pin, bindings::LOW as u8);
                }
                self.direction = Direction::Backward;
            }
            &Direction::Backward => {
                unsafe {
                    bindings::digitalWrite(self.direction_pin, bindings::HIGH as u8);
                }
                self.direction = Direction::Forward;
            }
        }
    }
}
