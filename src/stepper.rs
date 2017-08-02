use core::ops::Not;
use core::fmt::Debug;

use hardware::HardwareGpio;
use hardware::HardwareTime;
use hardware::PinMode;

const PULSE_LENGTH: u32 = 100;

#[derive(Debug)]
pub struct StepRateError {}

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

#[derive(Debug)]
pub struct StepperMotor<'a, H: 'a> {
    /// The current step that the motor is at
    current_step: i32,

    /// The current direction
    current_direction: Direction,

    /// The current traveling velocity in microseconds per step
    microseconds_per_step: u32,

    /// The number of steps per millimeter
    steps_per_millimeter: i32,

    /// The minimum of the stepper motor in mm
    min_travel: f32,

    /// The maximum of the stepper motor in mm
    max_travel: f32,

    /// The minimum number of steps that can be stepped
    min_steps: i32,

    /// The maxmimum number of steps that can be stepped
    max_steps: i32,

    /// The last time we stepped
    last_step: u32,

    /// If we are in the middle of a pulse
    mid_pulse: bool,

    /// The pin to use for stepping
    step_pin: u8,

    /// The pin to use for direction
    direction_pin: u8,

    /// The hardware to use to step the motor
    hardware: &'a mut H,
}

impl<'a, H: HardwareGpio + HardwareTime + Debug> StepperMotor<'a, H> {
    /// Make a new stepper motor
    pub fn new(
        steps_per_millimeter: i32,
        min_travel: f32,
        max_travel: f32,
        step_pin: u8,
        direction_pin: u8,
        hardware: &mut H,
    ) -> StepperMotor<H> {

        // Set the step and direction pins to output
        /*
        unsafe {
            bindings::pinMode(step_pin, bindings::OUTPUT as u8);
            bindings::pinMode(direction_pin, bindings::OUTPUT as u8);
        }
        */

        hardware.pin_mode(step_pin, PinMode::Output);
        hardware.pin_mode(direction_pin, PinMode::Output);

        StepperMotor {
            current_step: 0,
            current_direction: Direction::Backward,
            microseconds_per_step: 0,
            steps_per_millimeter: steps_per_millimeter,
            min_travel: min_travel,
            max_travel: max_travel,
            min_steps: (min_travel * steps_per_millimeter as f32) as i32,
            max_steps: (max_travel * steps_per_millimeter as f32) as i32,
            last_step: 0,
            mid_pulse: false,
            step_pin: step_pin,
            direction_pin: direction_pin,
            hardware: hardware,
        }
    }

    /// Get the max that this stepper can travel in mm
    pub fn get_max_travel(&self) -> f32 {
        self.max_travel
    }

    /// Get the min this stepper can travel in mm
    pub fn get_min_travel(&self) -> f32 {
        self.min_travel
    }

    /// Return the current position in mm
    pub fn get_current_position(&self) -> f32 {
        self.current_step as f32 / self.steps_per_millimeter as f32
    }

    /// Return the current velocity in mm/min
    pub fn get_current_velocity(&self) -> f32 {
        self.current_direction as i32 as f32 * 60000000.0 /
            (self.microseconds_per_step as i32 * self.steps_per_millimeter) as f32
    }

    /// Get the current direction
    pub fn get_current_direction(&self) -> Direction {
        self.current_direction
    }

    /// Set the max that this stepper can travel in mm
    pub fn set_max_travel(&mut self, max: f32) {
        self.max_travel = max;
        self.max_steps = (max * self.steps_per_millimeter as f32) as i32;
    }

    /// Set the min that this stepper can travel in mm
    pub fn set_min_travel(&mut self, min: f32) {
        self.min_travel = min;
        self.min_steps = (min * self.steps_per_millimeter as f32) as i32;
    }

    /// Set the current position (As in G92)
    /// Returns a `Result::Ok` with the now current step if successfull
    /// returns a `Result::Err` with the limit direction if the position is out of range
    pub fn set_current_position(&mut self, position: f32) -> Result<i32, Direction> {
        if position <= self.min_travel {
            Result::Err(Direction::Backward)
        } else if position >= self.max_travel {
            Result::Err(Direction::Forward)
        } else {
            self.current_step = (position * self.steps_per_millimeter as f32) as i32;
            Result::Ok(self.current_step)
        }
    }

    /// Set the current velocity in mm/min
    /// Returns a `Result::Ok` with the set microseconds per step if successful
    /// Retuens a `Result::Err` if the set speed is too fast to be able to step
    ///     The speed is not set in that case
    pub fn set_current_velocity(&mut self, velocity: f32) -> Result<u32, ()> {

        if velocity > 0.0 {
            self.set_current_direction(Direction::Forward);
            let microseconds_per_step =
                (60000000.0 / (velocity * self.steps_per_millimeter as f32)) as u32;

            if microseconds_per_step > PULSE_LENGTH {
                self.microseconds_per_step = microseconds_per_step;
                return Result::Ok(microseconds_per_step);
            } else {
                return Result::Err(());
            }

        } else if velocity < 0.0 {
            self.set_current_direction(Direction::Backward);
            let microseconds_per_step =
                (60000000.0 / (-velocity * self.steps_per_millimeter as f32)) as u32;

            if microseconds_per_step > PULSE_LENGTH {
                self.microseconds_per_step = microseconds_per_step;
                return Result::Ok(microseconds_per_step);
            } else {
                return Result::Err(());
            }

        } else {
            self.microseconds_per_step = 0;
            return Result::Ok(0);
        }
    }

    /// Set the current direction
    pub fn set_current_direction(&mut self, direction: Direction) {
        self.current_direction = direction;

        match direction {
            Direction::Forward => {
                //bindings::digitalWrite(self.direction_pin, bindings::HIGH as u8)
                self.hardware.digital_write(self.direction_pin, true);
            },
            Direction::Backward => {
                //bindings::digitalWrite(self.direction_pin, bindings::LOW as u8)
                self.hardware.digital_write(self.direction_pin, false);
            },
        }
    }

    /// Update everything
    /// Returns a `Result::Ok` with the current step if successfull,
    /// returns a `Result::Err` with the limit that would be breached
    ///     if the stepper would go out of range
    pub fn update(&mut self) -> Result<i32, Direction> {

        //let now = unsafe { bindings::micros() };
        let now = self.hardware.now();

        // Check if needed to start next step
        if now - self.last_step > self.microseconds_per_step {

            match self.current_direction {
                Direction::Forward => {
                    if self.current_step == self.max_steps {
                        return Result::Err(Direction::Forward);
                    }
                }
                Direction::Backward => {
                    if self.current_step == self.min_steps {
                        return Result::Err(Direction::Backward);
                    }
                }
            }

            /*
            unsafe {
                bindings::digitalWrite(self.step_pin, bindings::HIGH as u8);
            }
            */

            self.hardware.digital_write(self.step_pin, true);

            self.mid_pulse = true;
            self.last_step = now;
        }

        // Check if needed to end step pulse
        if self.mid_pulse && now - self.last_step > PULSE_LENGTH {

            /*
            unsafe {
                bindings::digitalWrite(self.step_pin, bindings::LOW as u8);
            }
            */

            self.hardware.digital_write(self.step_pin, false);

            self.current_step += self.current_direction as i32;
            self.mid_pulse = false;
            self.last_step = now;
        }

        Result::Ok(self.current_step)
    }
}
