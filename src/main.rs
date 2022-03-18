mod planet;

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures, mesh::VertexAttributeValues}, utils::tracing::span::Attributes,
};
use planet::Planet;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};

#[derive(Component)]
struct Rotator;

#[derive(Component, Inspectable)]
struct Noisify {
    #[inspectable(min = 0., max = 1.0)]
    max: f32,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_system(rotator_system)
        //.add_system(noise_system)
        .register_inspectable::<Noisify>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;
    // plane
    /*

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // sphere
    let mut planet = Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 10 });
    let initial = planet.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().clone();

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(planet),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 2.5, 0.0),
        ..Default::default()
    })
    .insert(Rotator)
    //.insert(Wireframe)
    .insert(Noisify { max: 0.0 })
    ;
     */

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Planet { radius: 2.0, resolution: 2 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
    .insert(Wireframe);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-5.0, 5.0, 5.0),
        Vec3::new(0., 0.0, 0.),
    ));
}

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.5  * time.delta_seconds());
    }
}

//fn noise_system(assets: Res<Assets<Mesh>>, time: Res<Time>, mut query: Query<(&Noisify, &Handle<Mesh>)>) {
fn noise_system(mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Noisify, &Handle<Mesh>), Changed<Noisify>>) {
    for (Noisify { max }, h) in query.iter() {
        println!("changed with a noise factor of {:?}", max);
        if max > &0.0 {
            if let Some(mesh) = meshes.get_mut(h) {
                if let Some(VertexAttributeValues::Float32x3(initial)) = mesh.attribute_mut("INIT_POSITION") {
                    let new_pos = initial.iter()
                    .map(|[x,y,z]| {
                        [x.clone() 
                        ,y.clone()
                        ,z.clone()]
                    })
                    .collect::<Vec<[f32; 3]>>();
                    //mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, new_pos);
                }
            }
        }
    }
}