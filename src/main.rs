use bevy::prelude::*;

mod color_change_background;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "aurora".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(player::PlayerPlugin)
        .add_plugins(color_change_background::ColorChangePlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
