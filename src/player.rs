use bevy::prelude::*;

use crate::*;

#[derive(Component, Debug)]
pub struct Player {
    pub v: f32,
    pub down: bool,
}

impl Player {
    pub fn new_bundle (asset_server: &Res<AssetServer>) -> (bevy::prelude::SpriteBundle, player::Player) {
        (
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture: asset_server.load("player.png"),
                transform: Transform::from_xyz(-300.0, 0.0, 0.0),
                ..default()
            }, 
            Player{v: 0.0, down: false},
        )
    }
}

pub fn player_update(
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>
) {
    for (mut texture, mut transform, mut p) in &mut query {
        if let crate::State::Home = game_state.state {
            if game_state.time > 2.0 {
                game_state.state = crate::State::Playing;
                game_state.time = 0.0;
                *transform = Transform::from_xyz(-300.0, 0.0, 0.0);
            }
        }

        if let crate::State::Playing = game_state.state {
            if transform.translation.x < 450.0 {
                if input.pressed(KeyCode::D) {
                    transform.translation.x +=  time.delta_seconds() * 100.0;
                }
            }
            if transform.translation.x > -450.0 {
                if input.pressed(KeyCode::A) {
                    transform.translation.x -=  time.delta_seconds() * 100.0;
                }
            }

            p.v -= 50.0;

            if transform.translation.y <= 0.0 {
                p.v = 0.0;
                transform.translation.y = 0.0;
                if input.pressed(KeyCode::W) {
                    p.v = 1000.0;
                }
                if input.pressed(KeyCode::S) {
                    *texture = asset_server.load("player_down.png");
                    p.down = true;
                }
                else {
                    *texture = asset_server.load("player.png");
                    p.down = false;
                }
                
            }

            let movement = time.delta_seconds() * p.v;
            transform.translation.y +=  movement;
            if transform.translation.y <= 0.0 {
                p.v = 0.0;
                transform.translation.y = 0.0;
            }
        }
        else
        {
            transform.translation.y = -200.0;
            transform.translation.x = 0.0;
        }
        game_state.time += time.delta_seconds();
    }
}