use bevy::{color::palettes::basic::RED, prelude::*};

#[derive(Component)]
pub struct Mino;


pub fn spawn_mino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mino,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(32.0)),
        GlobalTransform::default(),
    ));
}
