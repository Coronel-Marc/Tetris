use bevy::prelude::*;
mod viewer;
mod tetromino;
mod grid;

fn main() {
    App::new()
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
       // .add_plugins(tetromino::spawn_mino)
        .run();
    
}

