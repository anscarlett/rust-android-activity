//! Shared Bevy application logic.
//!
//! This crate contains the game/scene logic that is platform-independent.
//! Platform launchers (android-game-activity, android-native-activity, desktop)
//! depend on this crate and call [`add_app_plugins`] to wire everything into a
//! Bevy [`App`].
//!
//! # Adding more crates / modules
//!
//! Add a new entry to the workspace `Cargo.toml`, depend on it here (or
//! directly from a launcher), and compose its plugin(s) inside
//! [`BevyShapesPlugin::build`].

use std::f32::consts::PI;

use bevy::prelude::*;

/// Register all game plugins with the supplied [`App`].
///
/// Call this from every platform launcher after setting up window/backend
/// plugins.
pub fn add_app_plugins(app: &mut App) {
    app.add_plugins(BevyShapesPlugin);
}

/// Top-level plugin that sets up the Bevy 3D-shapes scene
/// (equivalent to Bevy's built-in `3d/shapes` example).
pub struct BevyShapesPlugin;

impl Plugin for BevyShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Update, rotate_shapes);
    }
}

// ---------------------------------------------------------------------------
// Marker component
// ---------------------------------------------------------------------------

/// Marker component for entities that should rotate each frame.
#[derive(Component)]
struct Shape;

// ---------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------

const NUM_SHAPES: usize = 6;
const X_EXTENT: f32 = 14.0;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Shared "debug" material – a plain white metallic look so the geometry
    // is clearly visible.
    let shape_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    });

    // The six shapes, matching Bevy's own `3d/shapes` example.
    let shape_meshes: [Handle<Mesh>; NUM_SHAPES] = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    for (i, mesh) in shape_meshes.into_iter().enumerate() {
        let x = -X_EXTENT / 2. + i as f32 / (NUM_SHAPES - 1) as f32 * X_EXTENT;
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(shape_material.clone()),
            Transform::from_xyz(x, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
        ));
    }

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.7))),
    ));

    // Point light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

fn rotate_shapes(time: Res<Time>, mut query: Query<&mut Transform, With<Shape>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}
