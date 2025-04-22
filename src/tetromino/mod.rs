pub mod types;
use bevy::{color::palettes::basic::RED, prelude::*};

use crate::tetromino::types::TetrominoType;
use crate::grid::{grid_to_world, GridPosition};

/* #[derive(Component)]
pub struct Mino; */

#[derive(Component)]
pub struct Tetromino{
    pub kind: TetrominoType,
}

pub fn spawn_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mesh = meshes.add(Rectangle::default());
    let material = materials.add(Color::from(RED));
    let kind = TetrominoType::O;

    let parent = commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        Tetromino { kind },
    )).id();

    
    for pos in kind.shape() {
        commands.spawn((
            GridPosition {x: pos.x, y: pos.y},
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(grid_to_world(pos.x,pos.y)).with_scale(Vec3::splat(32.0)),
            Visibility::Visible,
            InheritedVisibility::default(),
            GlobalTransform::default(),
        )).set_parent(parent);
    }
    println!("Spawned tetromino of kind {:?}", kind);
}



/* pub fn spawn_mino(
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
} */

fn spawn_tetromino_on_key(
    keys: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
){
    if keys.just_pressed(KeyCode::Space) {
        spawn_tetromino(commands, meshes, materials);
    }
}

pub struct TetrominoPlugin;
impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tetromino_on_key);
    }
}
