use crate::graphics::color;
use crate::graphics::material::Material;
use crate::graphics::render::Render;
use crate::graphics::resource::Mesh;
use crate::math::Vector3d;
use crate::physics::Position;

#[derive(Default, Debug)]
#[repr(C, align(16))]
pub struct MeshInfo {
    pub color: Vector3d,
}

#[derive(Clone)]
pub struct Entity {
    pub mesh: Mesh,
    pub materials: Vec<Material>,

    pub position: Position,
    pub color: Vector3d,
}

impl Entity {
    pub fn new(
        mesh: Mesh,
        materials: impl IntoIterator<Item = Material>,
        position: Position,
    ) -> Self {
        let materials: Vec<_> = materials.into_iter().collect();
        Self {
            mesh,
            materials,
            position,
            color: color::WHITE.into(),
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        self.position.update(delta_t);
    }

    pub fn get_mesh_and_materials<'a, 'b>(
        &'a mut self,
        render: &'b Render,
    ) -> (&'a mut Mesh, &'a mut [Material]) {
        for material in &mut self.materials {
            //Datum 1 is position. How to label?
            material
                .set_data(render, 1, &mut self.position.get_matrix())
                .unwrap();
            //Datum 2 is color. Mostly unused
            material
                .set_data(render, 2, &mut MeshInfo { color: self.color })
                .unwrap();
        }

        (&mut self.mesh, &mut self.materials)
    }
}
