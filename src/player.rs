use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (animate_sprite, player_movement));
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/aurora/aurora_run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    commands.spawn((
        SpriteSheetBundle {
            visibility: Visibility::Hidden,
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                scale: Vec3::new(4.0, 4.0, 0.0),
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

fn player_movement(
    mut player: Query<(
        &mut Transform,
        &mut Visibility,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut AnimationIndices,
        With<Player>,
    )>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const SPEED: f32 = 250.0;
    for (mut transform, mut visability, mut texture, mut sprite, mut animation_indices, _) in
        &mut player
    {
        if input.pressed(KeyCode::F) && input.pressed(KeyCode::S) {
        } else if input.pressed(KeyCode::F) {
            transform.translation.x += SPEED * time.delta_seconds();
            transform.scale = Vec3::new(4.0, 4.0, 0.0)
        } else if input.pressed(KeyCode::S) {
            transform.translation.x -= SPEED * time.delta_seconds();
            transform.scale = Vec3::new(-4.0, 4.0, 0.0)
        }
        if input.just_pressed(KeyCode::Key1) {
            *visability = Visibility::Visible;
        }
        if input.just_pressed(KeyCode::Key2) {
            *visability = Visibility::Hidden;
        }
        if input.just_pressed(KeyCode::Space) {
            *animation_indices = AnimationIndices { first: 0, last: 0 };
            *sprite = TextureAtlasSprite::new(animation_indices.first);
            let texture_handle = asset_server.load("sprites/aurora/aurora.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 1, None, None);
            *texture = texture_atlases.add(texture_atlas);
        }
        if input.just_pressed(KeyCode::ShiftLeft) {
            *animation_indices = AnimationIndices { first: 0, last: 3 };
            *sprite = TextureAtlasSprite::new(animation_indices.first);
            let texture_handle = asset_server.load("sprites/aurora/aurora_run.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1, None, None);
            *texture = texture_atlases.add(texture_atlas);
        }
    }
}
