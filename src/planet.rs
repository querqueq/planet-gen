use std::ops::{Add, Mul};
use bevy::render::{mesh::{Indices, Mesh, VertexAttributeValues}, render_resource::PrimitiveTopology};
use bevy::math::*;
use enum_iterator::IntoEnumIterator;
use self::Direction::*;

pub struct Planet {
    /// The radius of the sphere.
    pub radius: f32,
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

impl From<Planet> for Mesh {
    fn from(planet: Planet) -> Self {
        let resolution = planet.resolution.clone() as i64;
        let radius = &planet.radius;
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for direction in Direction::into_enum_iter() {
           add_face(&mut positions, &mut normals,&mut uvs, &mut indices, &resolution, radius, &direction.normal_vector());
        }
        println!("{:?}", positions);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
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
    println!("{} / {} = {}", length, resolution, sub);
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