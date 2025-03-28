use bevy::prelude::*;

mod window;
mod chunk;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, window::window::setup_camera)
        //.add_systems(Update, window::window::setup_camera)
        .run();
    
}

