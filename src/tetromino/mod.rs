pub mod types;


use bevy::input::keyboard::*;
use bevy::{color::palettes::basic::RED, prelude::*};

use crate::grid::constants::{GRID_WIDTH, GRID_HEIGHT};
use crate::tetromino::types::TetrominoType;
use crate::grid::{grid_to_world, GridPosition};

use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(Component)]
pub struct ActiveTetromino;


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
        )).set_parent(parent).insert(ActiveTetromino);

        println!("Spawned tetromino of kind {:?} at top", kind);
    }
}



fn spawn_tetromino_on_key(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<ActiveTetromino>>
){
    if keys.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_tetromino(commands, meshes, materials);
    }
}

pub struct TetrominoPlugin;
impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_tetromino_on_key,
            rotate_active_tetromino
        ));
    }
}

fn rotate_active_tetromino(
    keys: Res<ButtonInput<KeyCode>>,
    mut minos_query: Query<(&mut GridPosition, &mut Transform), With<ActiveTetromino>>,
    tetromino_query: Query<(&Children, &Tetromino), Without<ActiveTetromino>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        for (children, tetromino) in tetromino_query.iter() {
            let center = tetromino.kind.rotation_center();

            // Primeiro, calcula a nova posição de todos os minos
            let mut new_positions = vec![];
            for &child in children.iter() {
                if let Ok((pos, _)) = minos_query.get(child) {
                    let dx = pos.x as f32 - center.x;
                    let dy = pos.y as f32 - center.y;

                    let new_x = center.x - dy;
                    let new_y = center.y + dx;

                    new_positions.push(GridPosition {
                        x: new_x.round() as i32,
                        y: new_y.round() as i32,
                    });
                }
            }

            // Verifica se todos os novos minos estão dentro do grid
            let is_valid = new_positions.iter().all(|pos| {
                pos.x >= 0 && pos.x < GRID_WIDTH && pos.y >= 0 && pos.y < GRID_HEIGHT
            });

            if is_valid {
                // Se for válido, atualiza a posição de cada mino
                for (&child, new_pos) in children.iter().zip(new_positions.iter()) {
                    if let Ok((mut pos, mut transform)) = minos_query.get_mut(child) {
                        pos.x = new_pos.x;
                        pos.y = new_pos.y;
                        transform.translation = grid_to_world(pos.x, pos.y);
                    }
                }
                println!("Tetromino {:?} rotacionado com sucesso", tetromino.kind);
            } else {
                println!("Rotação inválida para o tetromino {:?}", tetromino.kind);
            }
        }
    }
}

