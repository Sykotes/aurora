use bevy::prelude::*;
use core::f32::consts::PI;

pub struct ColorChangePlugin;

impl Plugin for ColorChangePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColourChangeTime(Timer::from_seconds(
            12.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, change_color);
    }
}

#[derive(Resource)]
struct ColourChangeTime(Timer);

fn change_color(
    mut clear_color: ResMut<ClearColor>,
    time: Res<Time>,
    mut colour_change_timer: ResMut<ColourChangeTime>,
) {
    colour_change_timer.0.tick(time.delta());

    let divide_val = colour_change_timer.0.duration().as_secs() as f32 / 2.0;
    let sin_value = colour_change_timer.0.elapsed_secs() / divide_val;
    let color_transition_speed = 8.0;

    let red = (f32::sin(color_transition_speed * sin_value + 1.0)) / 2.0;
    let green = (f32::sin(color_transition_speed * sin_value + (2.0 * PI / 3.0)) + 1.0) / 2.0;
    let blue = (f32::sin(color_transition_speed * sin_value + (4.0 * PI / 3.0)) + 1.0) / 2.0;

    clear_color.set_r(red);
    clear_color.set_g(green);
    clear_color.set_b(blue);
}
