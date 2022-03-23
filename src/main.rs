mod planet;

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures, mesh::VertexAttributeValues}, utils::tracing::span::Attributes,
};
use planet::PlanetPlugin;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

//TODO put all settings for planet in its own data thingy
//     can use a vec of stuff and press add and remove in egui
//     could use that for the noise layers
//     can use a enum to pick a specific noise function
//     can send an event with InspectableButton to save settings
//     should implement Default to get initial values for noise layer (maybe load last saved)
//     should provide access to ambient light ResourceInspector<AmbientLight>

#[derive(Component, Inspectable)]
struct Rotator {
    rotate: bool
}

#[derive(Component, Inspectable)]
struct Noisify {
    #[inspectable(min = 0., max = 1.0)]
    max: f32,
}

#[derive(Component, Inspectable)]
struct Wireframed {
    wireframe: bool
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
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .add_plugin(PlanetPlugin)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(wireframe_system)
        //.add_system(noise_system)
        .register_inspectable::<Noisify>()
        .register_inspectable::<Rotator>()
        .register_inspectable::<Wireframed>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;

    commands.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        visibility: Visibility { is_visible: false },
        ..Default::default()
    });
/* 
    let radius = 5.0;
    let resolution = 60;
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(0.4, 0.3, 0.8, 1.),
        reflectance: 0.,
        metallic: 0.,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList)),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 2., 0.0),
        ..Default::default()
    })
    .insert(Planet { radius: radius, resolution: resolution, noise: None })
    .insert(Rotator { rotate: true })
    .insert(Wireframed { wireframe: false })
    ;
*/
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-20.0, 5.0, 5.0),
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

fn wireframe_system(mut commands: Commands, mut query: Query<(Entity, &Wireframed), (Changed<Wireframed>, With<Handle<Mesh>>)>) {
    for (entity_id, Wireframed { wireframe }) in query.iter() {
        let mut entity = commands.entity(entity_id);
        if *wireframe {
            entity.insert(Wireframe);
        } else {
            entity.remove::<Wireframe>();
        }
    }
}