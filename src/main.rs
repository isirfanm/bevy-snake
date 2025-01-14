use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins)
        .run();
}
