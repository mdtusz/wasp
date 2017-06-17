
use utils::Point3;

pub enum Command {
    Move,
    Temperature,

}

struct Printer {
    current_pos: Point3,
    command_queue: [Command; 32]
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
