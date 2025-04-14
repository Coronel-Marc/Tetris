//viewer/mod.rs
use bevy::prelude::*;
    
pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera (mut commands: Commands){
    commands.spawn(Camera2d::default());
}