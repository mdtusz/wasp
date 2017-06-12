use teensy3::bindings;

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
pub struct StepperMotorController {
    current_position: i32,
    current_velocity: i32,
    current_acceleration: i32,

    target_position: i32,
    target_velocity: i32,
    target_acceleration: i32,

    stop_acceleration_position: i32,
    start_acceleration_position: i32,

    stepper_motor: StepperMotor,
}

impl StepperMotorController {
    pub fn new(stepper_motor: StepperMotor) -> StepperMotorController {
        StepperMotorController {
            current_position: 0,
            current_velocity: 0,
            current_acceleration: 0,
            target_position: 0,
            target_velocity: 0,
            target_acceleration: 0,
            stop_acceleration_position: 0,
            start_acceleration_position: 0,

            stepper_motor: stepper_motor,
        }
    }

    pub fn set_target_position(&mut self, target_position: i32) {
        self.target_position = target_position;
    }

    pub fn set_target_velocity(&mut self, target_velocity: i32) {
        self.target_velocity = target_velocity;
    }

    pub fn set_target_acceleration(&mut self, target_acceleration: i32) {
        self.target_acceleration = target_acceleration;
    }

    pub fn update(&mut self) {
        
    }
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
