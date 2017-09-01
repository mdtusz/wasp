
use core::ops::Not;
use hardware::peripherals::digital_io::DigitalOutput;
use hardware::peripherals::digital_io::DigitalValue;
use hardware::peripherals::time::Time;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward = 1,
    Backward = -1,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Forward
    }
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
    fn set_velocity(&mut self, velocity: f32);
    fn set_direction(&mut self, direction: Direction);
    fn get_position(&self) -> f32;
    fn get_velocity(&self) -> f32;
    fn update(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct StepperMotorConfig {
    pub min_travel: f32,
    pub max_travel: f32,

    pub steps_per_millimeter: i32,

    pub pulse_length: u32,
}

pub struct StepperMotor<'a> {
    step_output: &'a mut DigitalOutput,
    dir_output: &'a mut DigitalOutput,

    time: &'a Time,

    config: StepperMotorConfig,

    /// The current step that the motor is at
    current_step: i32,

    /// The current direction
    current_direction: Direction,

    /// The current traveling velocity in microseconds per step
    microseconds_per_step: u32,

    /// The minimum number of steps that can be stepped
    min_steps: i32,

    /// The maxmimum number of steps that can be stepped
    max_steps: i32,

    /// The last time we stepped
    last_step: u32,

    /// If we are in the middle of a pulse
    mid_pulse: bool,
}

impl<'a> StepperMotor<'a> {
    pub fn new(
        step_output: &'a mut DigitalOutput,
        dir_output: &'a mut DigitalOutput,
        time: &'a Time,
        config: StepperMotorConfig,
    ) -> StepperMotor<'a> {
        StepperMotor {
            step_output: step_output,
            dir_output: dir_output,
            time: time,
            config: config,
            current_step: 0,
            current_direction: Direction::Backward,
            microseconds_per_step: 0,
            min_steps: (config.min_travel * config.steps_per_millimeter as f32) as i32,
            max_steps: (config.max_travel * config.steps_per_millimeter as f32) as i32,
            last_step: 0,
            mid_pulse: false,
        }
    }
}

impl<'a> Motor for StepperMotor<'a> {
    fn set_velocity(&mut self, velocity: f32) {
        
        if velocity > 0.0 {
            self.set_direction(Direction::Forward);
            let microseconds_per_step =
                (60000000.0 / (velocity * self.config.steps_per_millimeter as f32)) as u32;

            self.microseconds_per_step = microseconds_per_step;

        } else if velocity < 0.0 {
            self.set_direction(Direction::Backward);
            let microseconds_per_step =
                (60000000.0 / (-velocity * self.config.steps_per_millimeter as f32)) as u32;

            self.microseconds_per_step = microseconds_per_step;

        } else {
            self.microseconds_per_step = 0;
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.current_direction = direction;
        match self.current_direction {
            Direction::Forward => self.dir_output.write(DigitalValue::High),
            Direction::Backward => self.dir_output.write(DigitalValue::Low),
        }
    }

    fn get_position(&self) -> f32 {
        self.current_step as f32 / self.config.steps_per_millimeter as f32
    }

    /// Return the current velocity in mm/min
    fn get_velocity(&self) -> f32 {
        self.current_direction as i32 as f32 * 60000000.0 /
            (self.microseconds_per_step as i32 * self.config.steps_per_millimeter) as f32
    }

    fn update(&mut self) {
        //let now = unsafe { bindings::micros() };
        let now = self.time.now();

        // Check if needed to start next step
        if now - self.last_step > self.microseconds_per_step {
            if match self.current_direction {
                Direction::Forward => self.current_step != self.max_steps,
                Direction::Backward => self.current_step != self.min_steps,
            } {
                //self.hardware.digital_write(self.config.step_pin, PinState::High);
                self.step_output.write(DigitalValue::High);

                self.mid_pulse = true;
                self.last_step = now;
            }
        }

        // Check if needed to end step pulse
        if self.mid_pulse && now - self.last_step > self.config.pulse_length {

            //self.hardware.digital_write(self.config.step_pin, PinState::Low);
            self.step_output.write(DigitalValue::High);

            self.current_step += self.current_direction as i32;
            self.mid_pulse = false;
            self.last_step = now;
        }
    }
}
