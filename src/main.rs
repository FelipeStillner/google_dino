use bevy::prelude::*;
use cactus::*;
use player::*;
use ground::*;
use collision::*;
use bird::*;
use rand::Rng;

mod player;
mod ground;
mod cactus;
mod collision;
mod bird;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_update, cactus_update, check_colision_cactus, update, bird_update, check_colision_bird))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Google Dino".into(),
                    resolution: (960.0, 480.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build()
        )   
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameState{state: State::Home, time: 0.0, enemy: true});
    commands.spawn(Player::new_bundle(&asset_server));
    commands.spawn(Cactus::new_bundle(&asset_server));
    commands.spawn(Ground::new_bundle(&asset_server, -500.0));
    commands.spawn(Ground::new_bundle(&asset_server, -400.0));
    commands.spawn(Ground::new_bundle(&asset_server, -300.0));
    commands.spawn(Ground::new_bundle(&asset_server, -200.0));
    commands.spawn(Ground::new_bundle(&asset_server, -100.0));
    commands.spawn(Ground::new_bundle(&asset_server, 0.0));
    commands.spawn(Ground::new_bundle(&asset_server, 100.0));
    commands.spawn(Ground::new_bundle(&asset_server, 200.0));
    commands.spawn(Ground::new_bundle(&asset_server, 300.0));
    commands.spawn(Ground::new_bundle(&asset_server, 400.0));
    commands.spawn(Ground::new_bundle(&asset_server, 500.0));
    
}

pub fn update(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if !game_state.enemy {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            commands.spawn(Cactus::new_bundle(&asset_server));
        }
        else {
            commands.spawn(Bird::new_bundle(&asset_server));
        }
        game_state.enemy = true;
    }
}

#[derive(Resource)]
pub struct  GameState {
    state: State,
    time: f32, 
    enemy: bool
}

enum State {
    Playing,
    Home
}