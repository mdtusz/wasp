
use stepper::StepperMotorController;
use utils::Point3;

#[derive(Debug)]
pub struct CartesianMotionPlanner {
    x_motor: StepperMotorController,
    y_motor: StepperMotorController,
    z_motor: StepperMotorController,
}

impl CartesianMotionPlanner {
    fn new(x_motor: StepperMotorController,
           y_motor: StepperMotorController,
           z_motor: StepperMotorController)
           -> CartesianMotionPlanner {
        CartesianMotionPlanner {
            x_motor: x_motor,
            y_motor: y_motor,
            z_motor: z_motor,
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

    fn get_current_acceleration(&self) -> Point3 {
        Point3::new(self.x_motor.get_current_acceleration(),
                    self.y_motor.get_current_acceleration(),
                    self.z_motor.get_current_acceleration())
    }

    fn add_target(&mut self, point: Point3, feed_rate: f32) {
        let current_position = self.get_current_position();
        
    }

}

