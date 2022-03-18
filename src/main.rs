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

#[derive(Component, Inspectable)]
struct Rotator {
    rotate: bool
}

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
        .register_inspectable::<Rotator>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let radius = 2.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Planet { radius: radius, resolution: 2 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(-4.0, 2., -4.0),
        ..Default::default()
    })
    .insert(Rotator { rotate: true })
    ;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Planet { radius: radius, resolution: 3 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 2., 0.0),
        ..Default::default()
    })
    .insert(Rotator { rotate: true })
    ;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Planet { radius: radius, resolution: 4 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(4.0, 2., 4.0),
        ..Default::default()
    })
    .insert(Rotator { rotate: true })
    ;
    

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

fn rotator_system(time: Res<Time>, mut query: Query<(&Rotator, &mut Transform)>) {
    for (Rotator { rotate, ..}, mut transform) in query.iter_mut() {
        if *rotate {
            transform.rotation *= Quat::from_rotation_y(0.5  * time.delta_seconds());
        }
    }
}
