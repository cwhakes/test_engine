use engine::components::Camera;
use engine::graphics::resource::mesh::Mesh;
use engine::input::{self, Listener};
use engine::math::{Matrix4x4, Point, Vector3d, Vector4d};
use engine::time::DeltaT;

static SPEED: f32 = 0.5;

#[derive(Default)]
pub struct World {
    screen_width: f32,
    screen_height: f32,

    delta_t: DeltaT,
    pub rot_x: f32,
    pub rot_y: f32,
    pub scale_cube: f32,
    pub forward: f32,
    pub rightward: f32,
    world_matrix: Matrix4x4,
    pub camera: Camera,
    pub light_source: Matrix4x4,

    meshes: Vec<(Matrix4x4, Mesh)>,
}

#[derive(Default, Debug)]
pub struct Environment {
    view: Matrix4x4,
    proj: Matrix4x4,
    light_dir: Vector4d,
    camera_pos: Vector4d,
}

#[derive(Default, Debug)]
pub struct MeshInfo {
    pub position: Matrix4x4,
    pub color: Vector3d,
}

impl World {
    pub fn new() -> World {
        let mut camera = Camera::default();
        camera.move_forward(-1.0);
        let light_source = Matrix4x4::rotation_x(-std::f32::consts::PI / 6.0);

        World {
            scale_cube: 1.0,
            camera,
            light_source,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.delta_t.update();
        
        self.light_source *= Matrix4x4::rotation_y(1.0 * self.delta_t.get());

        self.camera
            .set_rotation(self.rot_x, self.rot_y)
            .move_forward(SPEED * self.forward * self.delta_t.get())
            .move_rightward(SPEED * self.rightward * self.delta_t.get());

    }

    pub fn environment(&self) -> Environment {
        let mut world = self.world_matrix.clone();
        world *= Matrix4x4::scaling([self.scale_cube, self.scale_cube, self.scale_cube]);

        let view = self.camera.get_view();

        let proj = Matrix4x4::perspective(
            std::f32::consts::PI/2.0,
            self.screen_width / self.screen_height,
            0.01,
            100.0
        );

        let light_dir = self.light_source.get_direction_z().to_4d(0.0);
        let camera_pos = self.camera.get_position();

        Environment {
            view,
            proj,
            light_dir,
            camera_pos,
        }
    }

    pub fn set_screen_size(&mut self, (width, height): (u32, u32)) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;
    }

    pub fn add_mesh(&mut self, position: Matrix4x4, mesh: Mesh) {
        self.meshes.push((position, mesh))
    }

    pub fn meshes(&self) -> impl Iterator<Item=&(Matrix4x4, Mesh)> {
        self.meshes.iter()
    }
}

impl Listener for World {
    fn name(&self) -> String {
        "World".to_string()
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => self.forward = 1.0,
            b'S' => self.forward = -1.0,
            b'A' => self.rightward = -1.0,
            b'D' => self.rightward = 1.0,
            _ => {}
        }
    }
    fn on_key_up(&mut self, _key: usize) {
        self.forward = 0.0;
        self.rightward = 0.0;
    }
    fn on_mouse_move(&mut self, pos: Point) {
        let (width, height) = (self.screen_width as i32, self.screen_height as i32);

        self.rot_x += (pos.y - height / 2) as f32 * 0.002;
        self.rot_y += (pos.x - width / 2) as f32 * 0.002;

        input::set_cursor_position((width / 2, height / 2));
    }
    fn on_left_mouse_down(&mut self) {
        self.scale_cube = 0.5
    }
    fn on_right_mouse_down(&mut self) {
        self.scale_cube = 1.5
    }
    fn on_left_mouse_up(&mut self) {
        self.scale_cube = 1.0
    }
    fn on_right_mouse_up(&mut self) {
        self.scale_cube = 1.0
    }
}
