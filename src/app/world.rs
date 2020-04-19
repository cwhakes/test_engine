pub struct World {
    delta_t: DeltaT,
    delta_pos: f32,
    delta_scale: f32,
    rot_x: f32,
    rot_y: f32,
    scale_cube: f32,
    forward: f32,
    rightward: f32,
    world_camera: Matrix4x4,
}