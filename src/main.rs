use bevy::prelude::*;
use bevy::time::Timer;

use grid::{GridPosition, grid_to_world};
use tetromino::ActiveTetromino;


mod viewer;
mod tetromino;
mod grid;


#[derive(Resource)]
struct GravityTimer(Timer);


fn main() {
    App::new()
        .add_systems(Update, gravity_system)
        .add_systems(Update, update_transform_system)
        .add_systems(Update, collision_with_floor_system)
        .insert_resource(
            GravityTimer(
                Timer::from_seconds(
                    0.5, TimerMode::Repeating
                )
            )
        )
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Just another Tetris".into(),
                    resolution: (500., 800.,).into(),
                    resizable: false,
                    ..default()
                 }),
                 ..default()
            }))
        .add_plugins(viewer::ViewerPlugin)
        .add_plugins(grid::GridPlugin)
        .add_plugins(tetromino::TetrominoPlugin)
        .run();
    
}

fn gravity_system(
    time: Res<Time>,
    mut timer: ResMut<GravityTimer>,
    mut query: Query<&mut GridPosition, With<ActiveTetromino>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut pos in query.iter_mut() {
            pos.y -= 1;
        }
    }
}

fn update_transform_system(
    mut query: Query<(&GridPosition, &mut Transform)>,
) {
    // Atualiza a posição do transform com base na posição da grade
    // para cada entidade com GridPosition e Transform
    // Isso deve ser feito a cada quadro para garantir que a posição
    // do transform esteja sempre atualizada
    // com a posição da grade.

    for (grid_pos, mut transform) in query.iter_mut() {
        transform.translation = grid_to_world(grid_pos.x, grid_pos.y);
    }
}

fn collision_with_floor_system(
    mut commands: Commands,
    query: Query<(Entity, &GridPosition), With<ActiveTetromino>>,
) {
    // Verifica se qualquer bloco do tetromino ativo encostou no chão
    let hit_floor = query.iter().any(|(_, pos)| pos.y <= 0);

    // Se sim, remove o tetromino ativo
    // e "congela" o tetromino removendo o marcador de ativo
    if hit_floor {
        for (entity, _) in query.iter() {
            // "Congela" o tetromino removendo o marcador de ativo
            commands.entity(entity).remove::<ActiveTetromino>();
        }
    }
}
