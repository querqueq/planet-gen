use std::ops::{Add, Mul};
use bevy::{render::{mesh::{Indices, Mesh, VertexAttributeValues, self}, render_resource::PrimitiveTopology}, prelude::*};
use bevy::math::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, egui::util::cache::CacheTrait};
use enum_iterator::IntoEnumIterator;
use self::Direction::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .register_inspectable::<Planet>()
        .add_system(planet_update_system)
        ;
    }
}

fn planet_update_system(mut meshes: ResMut<Assets<Mesh>>, mut query: Query<(Entity, &Planet, &Handle<Mesh>), Changed<Planet>>) {
    for (entity, planet, mesh_h) in query.iter() {
        if let Some(mesh) = meshes.get_mut(mesh_h) {
            planet.update(mesh);
        } else {
            println!("Missing mesh for planet {:?}", entity);
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Planet {
    #[inspectable(min = 0.1)]
    pub radius: f32,
    #[inspectable(min = 1)]
    pub resolution: u32,
}

#[repr(usize)]
#[derive(Debug, PartialEq, IntoEnumIterator, Copy, Clone)]
enum Direction {
    Up, 
    Down,
    Left,
    Right,
    Back,
    Forward
}

impl Direction {
    fn normal_vector(&self) -> Vec3 {
        match self {
            Up => Vec3::new(0., 1., 0.),
            Down => Vec3::new(0., -1., 0.),
            Left => Vec3::new(-1., 0., 0.),
            Right => Vec3::new(1., 0., 0.),
            Back => Vec3::new(0., 0., -1.),
            Forward => Vec3::new(0., 0., 1.),
        }
    }
}

impl Planet {
    fn update(&self, mesh: &mut Mesh) {
        let resolution = self.resolution.clone() as i64;
        let radius = &self.radius;
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for direction in Direction::into_enum_iter() {
           add_face(&mut positions, &mut normals,&mut uvs, &mut indices, &resolution, radius, &direction.normal_vector());
        }
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}

impl From<Planet> for Mesh {
    fn from(planet: Planet) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        planet.update(&mut mesh);
        mesh
    }
}

fn add_face(positions: &mut Vec<[f32; 3]>
    , normals: &mut Vec<[f32; 3]>
    , uvs: &mut Vec<[f32; 2]>
    , indices: &mut Vec<u32>
    , resolution: &i64
    , length: &f32
    , local_up: &Vec3) {
    let sub = length / *resolution as f32;
    let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
    let axis_b = local_up.cross(axis_a);
    let res = *resolution;
    let offset = res as u32 * 2;
    let heigth = local_up.mul(*length);
    for b in (-res)..(res + 1) {
        for a in (-res)..(res + 1) {
            let triangle_start = positions.len() as u32;
            let point = heigth.add(axis_a.mul(a as f32).add(axis_b.mul(b as f32)).mul(sub)).normalize().mul(*length);
            positions.push(point.to_array());
            normals.push(local_up.project_onto(point).to_array());
            //FIXME what do I need here?
            uvs.push([0.,0.]);

            if b != res && a != res {
                //FIXME do the triang(res + 1) have to be in a certain order (clockwise/anti-clockwise)?
                indices.push(triangle_start);
                indices.push(triangle_start + 1);
                indices.push(triangle_start + offset + 2);
                indices.push(triangle_start);
                indices.push(triangle_start + offset + 2);
                indices.push(triangle_start + offset + 1);
            }
        }
    }
}