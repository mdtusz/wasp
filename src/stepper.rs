use teensy3::bindings;

const PULSE_LENGTH: u32 = 50;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward = 1,
    Backward = -1,
}

#[derive(Debug)]
pub struct StepperMotor {
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

    /// The last time we stepped
    last_step: u32,

    /// If we are in the middle of a pulse
    mid_pulse: bool,

    /// The pin to use for stepping
    step_pin: u8,

    /// The pin to use for direction
    direction_pin: u8,
}

impl StepperMotor {
    /// Make a new stepper motor
    pub fn new(steps_per_millimeter: i32,
               min_travel: f32,
               max_travel: f32,
               step_pin: u8,
               direction_pin: u8)
               -> StepperMotor {
        StepperMotor {
            current_step: 0,
            current_direction: Direction::Backward,
            microseconds_per_step: 0,
            steps_per_millimeter: steps_per_millimeter,
            min_travel: min_travel,
            max_travel: max_travel,
            last_step: 0,
            mid_pulse: false,
            step_pin: step_pin,
            direction_pin: direction_pin,
        }
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

    /// Set the current position (As in G92)
    pub fn set_current_position(&mut self, position: f32) {
        self.current_step = (position * self.steps_per_millimeter as f32) as i32;
    }

    /// Set the current velocity in mm/min
    pub fn set_current_velocity(&mut self, velocity: f32) {

        if velocity > 0.0 {
            self.set_current_direction(Direction::Forward);
            self.microseconds_per_step =
                (60000000.0 / (velocity * self.steps_per_millimeter as f32)) as u32;
        } else if velocity < 0.0 {
            self.set_current_direction(Direction::Backward);
            self.microseconds_per_step =
                (60000000.0 / (-velocity * self.steps_per_millimeter as f32)) as u32;
        } else {
            self.microseconds_per_step = 0;
        }
    }

    /// Set the current direction
    pub fn set_current_direction(&mut self, direction: Direction) {
        self.current_direction = direction;

        match direction {
            Direction::Forward => unsafe {
                bindings::digitalWrite(self.direction_pin, bindings::HIGH as u8)
            },
            Direction::Backward => unsafe {
                bindings::digitalWrite(self.direction_pin, bindings::LOW as u8)
            },
        }
    }

    /// Update everything
    pub fn update(&mut self) {

        unsafe {
            let now = bindings::micros();

            // Check if needed to start next step
            if now - self.last_step > self.microseconds_per_step {
                bindings::digitalWrite(self.step_pin, bindings::HIGH as u8);

                self.mid_pulse = true;

                self.last_step = now;
            }

            // Check if needed to end step pulse
            if self.mid_pulse && now - self.last_step > PULSE_LENGTH {
                bindings::digitalWrite(self.step_pin, bindings::LOW as u8);

                self.mid_pulse = false;

                self.last_step = now;
            }
        }
    }
}
