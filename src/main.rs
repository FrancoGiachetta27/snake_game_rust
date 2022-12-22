use bevy::prelude::*;
use bevy_inspector_egui::{self, WorldInspectorPlugin};

mod snake;

use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                title: "Snake Game".to_string(),
                ..default()
            },
            ..default()
        }))
        // Plugins
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(SnakePlugin)
        // Systems
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    //get window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let button = win_h / 2.0;

    commands.spawn(Camera2dBundle::default());
}
