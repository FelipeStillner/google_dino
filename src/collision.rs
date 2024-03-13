use bevy::prelude::*;
use crate::*;

pub fn check_colision_cactus(
    mut players: Query<(&Transform, With<Player>)>,
    cactuss: Query<(&Transform, &Cactus, Without<Player>)>,
    mut game_state: ResMut<GameState>,
) {
    for (transform1,_) in players.iter_mut() {
        for (transform2, _, _) in cactuss.iter() {
            let dx = (transform1.translation.x - transform2.translation.x).abs();
            let dy = (transform1.translation.y - transform2.translation.y).abs();
            if dx < 50.0 && dy < 75.0 {
                game_state.state = crate::State::Home;
                game_state.time = 0.0;
            }
        }
    }
}

pub fn check_colision_bird(
    mut players: Query<(&Transform, &Player)>,
    cactuss: Query<(&Transform, &Bird, Without<Player>)>,
    mut game_state: ResMut<GameState>,
) {
    for (transform1, p) in players.iter_mut() {
        for (transform2, _, _) in cactuss.iter() {
            let dx = (transform1.translation.x - transform2.translation.x).abs();
            if dx < 50.0 && !p.down {
                game_state.state = crate::State::Home;
                game_state.time = 0.0;
            }
        }
    }
}
