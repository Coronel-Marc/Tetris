use bevy::{ecs::bundle::Bundles, prelude::*, sprite::MaterialMesh2dBundle, text::cosmic_text::rustybuzz::shape};
use crate::grid::constants::{BLOCK_SIZE, GRID_HEIGHT};
pub mod constants;


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        //adcionar mais coisas já já...
        app.add_systems(Startup, spawn_grid);
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

pub fn grid_to_world(x:i32, y:i32) -> Vec3 {
    //O (0, 0) será o canto inferior esquerdo.
    let pixel_x = x as f32 * BLOCK_SIZE;
    let pixel_y = y as f32 * BLOCK_SIZE;
    Vec3::new(pixel_x, pixel_y, 0.0)
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for y in 0..GRID_HEIGHT {
        for x in 0..constants::GRID_WIDTH {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    material: materials.add(Color::rgba(
                        0.8, 0.8, 0.8, 0.1)), // cinza transparente
                    transform: Transform::from_translation(grid_to_world(x, y))
                        .with_scale(Vec3::splat(BLOCK_SIZE)),
                    ..default()
                },
                GridPosition { x, y },
            ));
        }
    }
}