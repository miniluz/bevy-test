#![allow(unused)]

use bevy::prelude::*;
use components::{Velocity, Movable};
use player::*;

mod components;
mod player;

// region:    --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

// endregion: --- Asset Constants

// region:    --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion: --- Game Constants

// region:    --- Resources

#[derive(Resource)]
pub struct WinSize{
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

// endregion: --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Invaders!".to_string(),
                width: 598.0,
                height: 676.0,
                ..default()
                },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .run();
}


fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add winsize
    let win_size = WinSize{w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // position_window
    window.set_position(MonitorSelection::Primary,IVec2::new(910, 0));

    // add GameTextures resource
    let game_textures = GameTextures{
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);

}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y >  win_size.h /2. + MARGIN 
            || translation.y < -win_size.h /2. - MARGIN
            || translation.x >  win_size.w /2. + MARGIN 
            || translation.x < -win_size.w /2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}