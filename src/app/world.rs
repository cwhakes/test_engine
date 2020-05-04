use super::Constant;

use engine::components::Camera;
use engine::graphics::render::shaders;
use engine::graphics::render::{ConstantBuffer, Context};
use engine::math::Matrix4x4;
use engine::time::DeltaT;

#[derive(Default)]
pub struct World {
    pub delta_t: DeltaT,
    pub delta_pos: f32,
    pub delta_scale: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub scale_cube: f32,
    pub forward: f32,
    pub rightward: f32,
    pub camera: Camera,
    //pub world_camera: Matrix4x4,
    pub light_source: Matrix4x4,
}

impl World {
    pub fn new() -> World {
        let mut camera = Camera::default();
        camera.move_forward(-1.0);

        World {
            scale_cube: 1.0,
            camera,
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        constant_buffer: &mut ConstantBuffer<Constant>,
        context: &Context,
        (width, height): (u32, u32),
    ) {
        
        self.delta_scale += self.delta_t.get() / 1.0;
        self.light_source *= Matrix4x4::rotation_y(1.0 * self.delta_t.get());

        let world = Matrix4x4::scaling([self.scale_cube, self.scale_cube, self.scale_cube]);

        self.camera
            .set_rotation(self.rot_x, self.rot_y)
            .move_forward(self.forward * 5.0)
            .move_rightward(self.rightward * 5.0);

        let view = self.camera.get_view();

        let proj = Matrix4x4::perspective(0.785, width as f32 / height as f32, 0.001, 100.0);

        let light_dir = self.light_source.get_direction_z().to_4d(0.0);
        //let camera_pos = self.world_camera.get_translation().to_4d(1.0);
        let camera_pos = self.camera.get_position();

        let mut constant = Constant {
            world,
            view,
            proj,
            light_dir,
            camera_pos,
        };
        constant_buffer.update(context, &mut constant);
        context.set_constant_buffer::<shaders::Vertex, _>(constant_buffer);
        context.set_constant_buffer::<shaders::Pixel, _>(constant_buffer);
    }
}
