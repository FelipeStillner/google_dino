use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Ground; 

impl Ground {
    pub fn new_bundle (asset_server: &Res<AssetServer>, positionx: f32) -> (bevy::prelude::SpriteBundle, Ground) {
        (
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture: asset_server.load("ground.png"),
                transform: Transform::from_xyz(positionx, -100.0, 0.0),
                ..default()
            }, 
            Ground{},
        )
    }
}