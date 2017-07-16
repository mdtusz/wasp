use core::fmt::Debug;
use math::sqrtf;
use stepper::StepperMotor;
use utils::Point3;
use teensy3::bindings;
use hardware::HardwareGpio;
use hardware::HardwareTime;

#[derive(Debug)]
pub struct CartesianMotionPlanner<'a, H: 'a> {
    x_motor: StepperMotor<'a, H>,
    y_motor: StepperMotor<'a, H>,
    z_motor: StepperMotor<'a, H>,
    hardware: &'a H,
    max_acceleration: f32,
    max_speed: f32,
    start_speed: f32,
    end_speed: f32,
    motion_distance: f32,
    delta_x: f32,
    delta_y: f32,
    delta_z: f32,
    top_x_speed: f32,
    top_y_speed: f32,
    top_z_speed: f32,
    start_time: u32,
    transition1: u32,
    transition2: u32,
    end_time: u32,
    set_start: bool,
}

impl<'a, H: HardwareGpio + HardwareTime + Debug> CartesianMotionPlanner<'a, H> {
    fn new(x_motor: StepperMotor<'a, H>,
           y_motor: StepperMotor<'a, H>,
           z_motor: StepperMotor<'a, H>,
           hardware: &'a H,
           max_acceleration: f32,
           max_speed: f32,
           start_speed: f32,
           end_speed: f32)
           -> CartesianMotionPlanner<'a, H> {

        CartesianMotionPlanner {
            x_motor: x_motor,
            y_motor: y_motor,
            z_motor: z_motor,
            hardware: hardware,
            max_acceleration: max_acceleration,
            max_speed: max_speed,
            start_speed: start_speed,
            end_speed: end_speed,
            motion_distance: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            delta_z: 0.0,
            top_x_speed: 0.0,
            top_y_speed: 0.0,
            top_z_speed: 0.0,
            start_time: 0,
            transition1: 0,
            transition2: 0,
            end_time: 0,
            set_start: false,
        }
    }

    fn get_current_position(&self) -> Point3 {
        Point3::new(self.x_motor.get_current_position(),
                    self.y_motor.get_current_position(),
                    self.z_motor.get_current_position())
    }

    fn get_current_velocity(&self) -> Point3 {
        Point3::new(self.x_motor.get_current_velocity(),
                    self.y_motor.get_current_velocity(),
                    self.z_motor.get_current_velocity())
    }

    // fn get_current_acceleration(&self) -> Point3 {
    // Point3::new(self.x_motor.get_current_acceleration(),
    // self.y_motor.get_current_acceleration(),
    // self.z_motor.get_current_acceleration())
    // }
    //

    fn set_target(&mut self, point: Point3) {
        let current_position = self.get_current_position();
        self.delta_x = point.x - current_position.x;
        self.delta_y = point.y - current_position.y;
        self.delta_z = point.z - current_position.z;
        self.motion_distance = sqrtf(self.delta_x * self.delta_x + self.delta_y * self.delta_y +
                                     self.delta_z * self.delta_z);
        self.top_x_speed = self.translate_to_axis(self.delta_x,
                                                  self.get_axis_top_speed(self.delta_x,
                                                                          self.max_speed));
        self.top_y_speed = self.translate_to_axis(self.delta_y,
                                                  self.get_axis_top_speed(self.delta_y,
                                                                          self.max_speed));
        self.top_z_speed = self.translate_to_axis(self.delta_z,
                                                  self.get_axis_top_speed(self.delta_z,
                                                                          self.max_speed));
        self.transition1 = ((self.max_speed - self.start_speed) / self.max_acceleration) as u32;
        let part1_distance = self.start_speed * self.transition1 as f32 +
                             (1.0 / 2.0) * self.max_acceleration *
                             (self.transition1 * self.transition1) as f32;
        let part3_time = ((self.max_speed - self.end_speed) / self.max_acceleration) as u32;
        let part3_distance = self.start_speed * part3_time as f32 +
                             (1.0 / 2.0) * self.max_acceleration * (part3_time * part3_time) as f32;
        self.transition2 =
            (self.motion_distance - (part1_distance + part3_distance) / self.max_speed) as u32;
        self.end_time = self.transition2 + part3_time;
        self.set_start = true;
    }

    fn update(&mut self) {
        if self.set_start {
            self.start_time = self.hardware.now();
            self.set_start = false;
        }
        let now = self.hardware.now() - self.start_time;
        // TODO account for now is negative?
        let current_speed;
        if now < self.transition1 {
            current_speed = now as f32 * self.max_acceleration + self.start_speed as f32;
        } else if now >= self.transition1 && now < self.transition2 {
            current_speed = self.max_speed;
        } else if now >= self.transition2 && now < self.end_time {
            current_speed = now as f32 * self.max_acceleration * -1.0 + self.max_speed;
        } else {
            current_speed = self.end_speed;
        }
        // TODO Handle set_current_velocity failure.
        self.x_motor.set_current_velocity((current_speed * self.delta_x) / self.motion_distance);
        self.y_motor.set_current_velocity((current_speed * self.delta_y) / self.motion_distance);
        self.z_motor.set_current_velocity((current_speed * self.delta_z) / self.motion_distance);
    }

    fn get_axis_top_speed(&self, delta: f32, top_speed: f32) -> f32 {
        let speedup_distance = top_speed / self.max_acceleration;
        if speedup_distance < delta / 2.0 {
            (delta / 2.0) * self.max_acceleration
        } else {
            top_speed
        }
    }

    #[inline]
    fn translate_to_axis(&self, delta: f32, axis_speed: f32) -> f32 {
        (axis_speed * delta) / self.motion_distance
    }
}
