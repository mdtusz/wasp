
use utils::Point3;

struct Printer {
    current_pos: Point3,
}

impl Printer {

    fn new() -> Printer {
        Printer {
            current_pos: Point3::new(0.0, 0.0, 0.0),
        }
    }
    
    fn move_to(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
    
    }

}
