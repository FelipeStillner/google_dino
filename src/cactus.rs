use bevy::prelude::*;
use crate::*;

#[derive(Component, Debug)]
pub struct Cactus; 

impl Cactus {
    pub fn new_bundle (asset_server: &Res<AssetServer>) -> (bevy::prelude::SpriteBundle, Cactus) {
        let mut rng = rand::thread_rng();
        (
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 100.0)),
                    ..default()
                },
                texture: asset_server.load("ground.png"),
                transform: Transform::from_xyz(500.0 + rng.gen_range(0..500) as f32, 0.0, 0.0),
                ..default()
            }, 
            Cactus{},
        )
    }
}

pub fn cactus_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Cactus)>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    for (ent, mut transform, _) in query.iter_mut() {
        transform.translation.x -= 250.0 * time.delta_seconds() * (game_state.time/2.0 + 1.0);
        if transform.translation.x < -550.0 {
            commands.entity(ent).despawn();
            game_state.enemy = false;
        }
    }
}