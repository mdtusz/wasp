
struct Configuration {
    
    x_axis_steps_per_mm: i32,
    y_axis_steps_per_mm: i32,
    z_axis_steps_per_mm: i32,

    x_axis_min_travel: f32,
    y_axis_min_travel: f32,
    z_axis_min_travel: f32,

    x_axis_max_travel: f32,
    y_axis_max_travel: f32,
    z_axis_max_travel: f32,

    x_axis_max_acceleration: f32,
    y_axis_max_acceleration: f32,
    z_axis_max_acceleration: f32,

    x_axis_max_feedrate: f32,
    y_axis_max_feedrate: f32,
    z_axis_max_feedrate: f32,
}