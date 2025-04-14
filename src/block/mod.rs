use bevy::{color::palettes::basic::RED, prelude::*};

pub fn setup_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::default().with_scale(Vec3::splat(64.)),
    ));
}