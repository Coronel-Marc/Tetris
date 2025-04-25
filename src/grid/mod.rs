//grid/mod.rs
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use crate::grid::constants::{BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH};
pub mod constants;


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_grid);
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct RotationCenter {
    pub x: f32,
    pub y: f32,
}


pub fn grid_to_world(x:i32, y:i32) -> Vec3 {
    //O (0, 0) será o canto inferior esquerdo.
    let offset_x = -(GRID_WIDTH as f32 * BLOCK_SIZE) / 2.0;
    let offset_y = -(GRID_HEIGHT as f32 * BLOCK_SIZE) / 2.0;

    let pixel_x = x as f32 * BLOCK_SIZE + offset_x + BLOCK_SIZE / 2.0;
    let pixel_y = y as f32 * BLOCK_SIZE + offset_y + BLOCK_SIZE / 2.0;
    Vec3::new(pixel_x, pixel_y, 0.0)
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let position = GridPosition { x, y };
            commands.spawn((
                position,
                Mesh2d(meshes.add(Rectangle::new(
                    BLOCK_SIZE, 
                    BLOCK_SIZE))),
                MeshMaterial2d(materials.add(Color::
                    srgba(0.8, 0.8, 0.8, 0.1))),
                Transform::from_translation(grid_to_world(x, y)),
            ));
        }
    }
}

/*
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::srgba(0.8, 0.8, 0.8, 0.1))),
                Transform::from_translation(grid_to_world(x, y))
                    .with_scale(Vec3::splat(BLOCK_SIZE))
*/