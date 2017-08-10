
use utils::Point3;
use motor::Motor;

pub trait MotionPlanner {
    fn set_target(&mut self, target: Point3);
}

pub struct CartesianMotionPlanner<'a> {
    target: Point3,
    x_motor: &'a mut Motor,
    y_motor: &'a mut Motor,
}

impl<'a> CartesianMotionPlanner<'a> {
    fn new(x_motor: &'a mut Motor, y_motor: &'a mut Motor) -> CartesianMotionPlanner<'a> {
        CartesianMotionPlanner {
            target: Point3::new(0.0, 0.0, 0.0),
            x_motor: x_motor,
            y_motor: y_motor,
        }
    }
}

impl<'a> MotionPlanner for CartesianMotionPlanner<'a> {
    fn set_target(&mut self, target: Point3) {}
}
