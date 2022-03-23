use std::ops::{Add, Mul};
use bevy::{render::{mesh::{Indices, Mesh, VertexAttributeValues, self}, render_resource::PrimitiveTopology}, prelude::*, pbr::wireframe::Wireframe};
use bevy::math::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, egui::util::cache::CacheTrait, InspectorPlugin};
use enum_iterator::IntoEnumIterator;
use rand::distributions::Open01;
use self::Direction::*;
use noise::{OpenSimplex, NoiseFn, Perlin};

pub struct PlanetPlugin;

#[derive(Component)]
struct Planet;

#[derive(Inspectable)]
pub struct PlanetSettings {
    #[inspectable(min = 0.2)]
    radius: f32,
    #[inspectable(min = 1)]
    resolution: u32,
    color: Color,
    #[inspectable(collapse)]
    noise_settings: NoiseSettings,
    wireframe: bool
}

#[derive(Clone, Inspectable)]
pub struct NoiseSettings {
    #[inspectable(min = 0.1, max = 3.0, speed = 0.1)]
    persistence: f32,
    #[inspectable(min = 0.0, max = 2.0)]
    base_roughness: f32,
    #[inspectable(min = 0.0, max = 5.0)]
    roughness: f32,
    #[inspectable(visual, min = Vec2::splat(-2.0), max = Vec2::splat(2.0))]
    offset: Vec2,
    layers: Vec<NoiseLayerSettings>,
}

#[derive(Clone, Inspectable)]
pub enum NoiseLayerSettings {
    Simplex {
        #[inspectable(min = 0.0)]
        frequency: f32,
        #[inspectable(min = 0.0)]
        amplitude: f64,
    },
    Perlin {
        #[inspectable(min = 0.0)]
        frequency: f32,
        #[inspectable(min = 0.0)]
        amplitude: f64,
    }
}


impl Default for PlanetSettings {
    fn default() -> Self {
        Self { 
            radius: 5., 
            resolution: 33, 
            color: Default::default(), 
            noise_settings: Default::default(),
            wireframe: false
        }
    }
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self { 
            persistence: Default::default(),
            base_roughness: Default::default(),
            roughness: Default::default(),
            offset: Default::default(), 
            layers: vec![NoiseLayerSettings::default()]
        }
    }
}

impl Default for NoiseLayerSettings {
    fn default() -> Self {
        NoiseLayerSettings::Simplex { 
            frequency: 1.0,
            amplitude: 1.0
         }
    }
}

#[derive(Default)]
struct Noises {
    perlin: Perlin,
    simplex: OpenSimplex
}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugin(InspectorPlugin::<PlanetSettings>::new())
        .add_system(planet_update_system)
        .add_startup_system(setup)
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(0.4, 0.3, 0.8, 1.),
        reflectance: 0.,
        metallic: 0.,
        ..Default::default()
    });

    commands.insert_resource(Noises::default());
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList)),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 2., 0.0),
        ..Default::default()
    })
    .insert(Planet)
    ;
}

fn planet_update_system(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    noises: Res<Noises>, 
    settings: Res<PlanetSettings>, 
    query: Query<(Entity, &Planet, &Handle<Mesh>, Option<&Wireframe>)>) {
    if settings.is_changed() {
        let noise_functions = &*noises;
        let planet_settings = &*settings;
        for (entity_id, planet, mesh_h, wireframe) in query.iter() {
            let mut entity = commands.entity(entity_id);
            if let Some(mesh) = meshes.get_mut(mesh_h) {
                planet.update(planet_settings, mesh, noise_functions);
            } else {
                println!("Missing mesh for planet {:?}", entity_id);
            }
            if wireframe.is_some() && !planet_settings.wireframe {
                entity.remove::<Wireframe>();
            } else if wireframe.is_none() && planet_settings.wireframe {
                entity.insert(Wireframe);
            }
        }
    }
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
    fn update(&self, planet_settings: &PlanetSettings, mesh: &mut Mesh, noise_functions: &Noises) {
        let resolution = planet_settings.resolution.clone() as i64;
        let radius = &planet_settings.radius;
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        //how to partial function application?!
        //let get_elevation = |p| noise_elevation(p, noise_functions, &planet_settings.noise_settings);
        for direction in Direction::into_enum_iter() {
            add_face(&mut positions, &mut normals,&mut uvs, &mut indices, noise_functions, &planet_settings.noise_settings, &resolution, radius, &direction.normal_vector());
        }
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}
/*
impl From<Planet> for Mesh {
    fn from(planet: Planet) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        planet.update(&mut mesh, NoNoise);
        mesh
    }
}
 */

fn add_face(positions: &mut Vec<[f32; 3]>
    , normals: &mut Vec<[f32; 3]>
    , uvs: &mut Vec<[f32; 2]>
    , indices: &mut Vec<u32>
    , noise_functions: &Noises
    , noise_settings: &NoiseSettings
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
            let point = heigth.add(axis_a.mul(a as f32).add(axis_b.mul(b as f32)).mul(sub)).normalize().mul(*length);//.add(local_up.mul(elevation));
            //scale elevation by planet radius otherwise changes will be to insignificat on biiiig planets
            let elevation = noise_elevation(&point, noise_functions, noise_settings);
            let normal = local_up.project_onto(point);
            let point = point.add(normal.mul(elevation));
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

fn noise_elevation(point: &Vec3, noise_functions: &Noises, noise_settings: &NoiseSettings) -> f32 {
    let mut elevation = 0.;
    for layer in noise_settings.layers.iter() {
        let noise = match layer {
            &NoiseLayerSettings::Simplex { frequency, amplitude } 
            => noise_functions.simplex.get(vec_to_array(point.mul(frequency))) * amplitude,
            &NoiseLayerSettings::Perlin { frequency, amplitude }
            => noise_functions.perlin.get(vec_to_array(point.mul(frequency))) * amplitude
        };
        elevation += noise;
    }

    (elevation as f32).max(0.)
}

fn vec_to_array(vec: Vec3) -> [f64; 3] {
    [vec.x as f64, vec.y as f64, vec.z as f64]
}