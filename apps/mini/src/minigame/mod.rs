mod world;

use engine::graphics::render::rendered_texture::Flavor;
use rand::{distributions::uniform::Uniform, prelude::*};
use shader::{DirectionalLight, Skybox};
use world::World;

use engine::components::Entity;
use engine::error::Result;
use engine::graphics::render::RenderedTexture;
use engine::graphics::GRAPHICS;
use engine::graphics::{color, Graphics};
use engine::math::{Matrix4x4, Point, Rect, Vector3d};
use engine::physics::Position;

use std::sync::Arc;

#[derive(Listener)]
pub struct MiniGame {
    rect: Rect<i32>,
    pub render_target: Arc<RenderedTexture>,
    depth_stencil: Arc<RenderedTexture>,

    #[listener]
    pub variables: World,

    _asteroids_pos: Vec<(Vector3d, Vector3d, Vector3d)>,
}

impl MiniGame {
    pub fn new(rect: Rect<i32>, graphics: &mut Graphics) -> Result<Self> {
        let device = &mut graphics.render.device_mut();

        let render_target =
            RenderedTexture::new((&rect).into(), Flavor::RenderTarget, device).unwrap();
        let depth_stencil =
            RenderedTexture::new((&rect).into(), Flavor::DepthStencil, device).unwrap();

        let mut world = World::new();

        let material = graphics.new_material::<DirectionalLight>()?;

        let spaceship = graphics.get_mesh_from_file("assets\\Meshes\\spaceship.obj")?;
        let mut spaceship_mat = material.clone();
        spaceship_mat
            .add_texture(graphics.get_texture_from_file("assets\\Textures\\spaceship.jpg")?);

        world.add_entity(
            "ship".into(),
            Entity::new(
                spaceship,
                vec![spaceship_mat],
                Position::new(Matrix4x4::translation([0.0, 0.0, 0.0])),
            ),
        );

        let mut sky_material = graphics.new_material::<Skybox>()?.with_frontface_culling();
        sky_material
            .add_texture(graphics.get_texture_from_file("assets\\Textures\\stars_map.jpg")?);

        let sky_mesh = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;

        world.add_sky_entity(Entity::new(
            sky_mesh,
            Some(sky_material),
            Position::default(),
        ));

        let mut asteroids_pos = Vec::new();

        let asteroid = graphics.get_mesh_from_file("assets\\Meshes\\asteroid.obj")?;
        let mut asteroid_mat = material;
        asteroid_mat.add_texture(graphics.get_texture_from_file("assets\\Textures\\asteroid.jpg")?);

        let mut rng = rand::thread_rng();
        let loc_range = Uniform::new(-2000.0, 2000.0);
        let rot_range = Uniform::new(0.0, std::f32::consts::TAU);
        let scale_range = Uniform::new(6.0, 30.0);
        for i in 0..200 {
            let loc = Vector3d::new(
                rng.sample(&loc_range),
                rng.sample(&loc_range),
                rng.sample(&loc_range),
            );
            let rot = Vector3d::new(
                rng.sample(&rot_range),
                rng.sample(&rot_range),
                rng.sample(&rot_range),
            );
            let scale = rng.sample(&scale_range);
            let scale = Vector3d::new(scale, scale, scale);

            let mut pos = Position::default();
            pos.set_postition(scale, rot, loc);

            world.add_entity(
                format!("asteroid_{}", i).into(),
                Entity::new(asteroid.clone(), vec![asteroid_mat.clone()], pos),
            );

            asteroids_pos.push((loc, rot, scale));
        }

        let mut app_window = Self {
            rect,
            render_target: Arc::new(render_target),
            depth_stencil: Arc::new(depth_stencil),
            variables: world,
            _asteroids_pos: asteroids_pos,
        };

        app_window
            .variables
            .set_screen_size(app_window.rect.clone());

        graphics.render.device().debug()?;

        Ok(app_window)
    }

    pub fn update(&mut self) {
        let mut g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(
            &mut (self.render_target.as_ref(), self.depth_stencil.as_ref()),
            color::NICE_BLUE,
        );
        context.set_render_target(&mut (self.render_target.as_ref(), self.depth_stencil.as_ref()));
        let (width, height) = self.rect.dims();
        context.set_viewport_size(width as f32, height as f32);

        self.variables.update();
        let mut environment = self.variables.environment();
        self.variables
            .set_environment_data(&g.render, &mut environment);

        for (mesh, materials) in self.variables.meshes_and_materials(&g.render) {
            g.render.draw_mesh_and_materials(mesh, materials);
        }
    }

    pub fn _on_resize(&mut self) {
        self.variables
            .set_screen_size(self.rect.clone() as Rect<i32>);
        if self.variables.play_state.is_playing() {
            self.variables.screen.center_cursor();
        }
    }
}
