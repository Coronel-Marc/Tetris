pub mod types;
use bevy::input::keyboard::*;
use bevy::{color::palettes::basic::RED, prelude::*};

use crate::grid::constants::{GRID_WIDTH, GRID_HEIGHT};
use crate::tetromino::types::TetrominoType;
use crate::grid::{grid_to_world, GridPosition};

use rand::seq::SliceRandom;
use rand::thread_rng;


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

    let types = [
        TetrominoType::I,
        TetrominoType::J,
        TetrominoType::L,
        TetrominoType::O,
        TetrominoType::S,
        TetrominoType::T,
        TetrominoType::Z,
    ];
    let kind = *types.choose(&mut thread_rng()).unwrap();

    let parent = commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        Tetromino { kind },
    )).id();

    let start_x = GRID_WIDTH / 2 - 1;
    let start_y = GRID_HEIGHT - 2;

    
    for pos in kind.shape() {
        let x = pos.x + start_x;
        let y = pos.y + start_y;

        commands.spawn((
            GridPosition {x, y},
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(grid_to_world(x,y)).with_scale(Vec3::splat(32.0)),
        )).set_parent(parent);

        println!("Spawned tetromino of kind {:?} at top", kind);
    }
}



fn spawn_tetromino_on_key(
    keys: ResMut<ButtonInput<KeyCode>>,
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
