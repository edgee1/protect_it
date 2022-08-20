#![allow(unused)]

use std::{iter, fs, time, thread};

use bevy::prelude::*;

use crate::{ENEMY_1_TEXTURE, Textures, setup, LEVEL_01_ENEMIES_ORDER, ChangeHealthEvent, EnemyDespawned};

pub struct EnemyPlugin;
#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_enemies)
            .add_system(enemy_movement)
            .add_system(debug_enemy_health);
            
    }
}
#[derive(Component)]
pub struct EnemyCharacteristics {
    texture: Handle<Image>,
    size: Vec2,
    speed: f32,
    health: f32
}
#[derive(Component)]
pub struct MarkedBy ( pub Vec<u32>);


impl EnemyCharacteristics {
    fn enemy_1 (
        asset_server: &AssetServer,
    ) -> Self {
        EnemyCharacteristics { 
        texture: asset_server.load("enemy_1.png"), 
        size: Vec2::new(50., 50.), 
        speed: 10., 
        health: 3. }
    }
}

pub fn spawn_enemies(    
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //closure that spawns an enemy
    let mut spawn_enemy = |characteristics: EnemyCharacteristics, position: f32| {
        commands.spawn_bundle(SpriteBundle{
            texture: characteristics.texture.clone(),
            transform: Transform { 
                translation: Vec3::new(position, 50., 10.) , 
                ..Default::default() },
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(characteristics)
        .insert(MarkedBy(Vec::new()));
    };

    //spawning distance
    let distance_between_enemies:f32= 25.;
    let distance_between_waves:f32= 500.;

    //spawning enemy wavws 
    let mut next_enemy_postion:f32 = -800. / 2. - 50.;
    let order_of_spawn_to_char = LEVEL_01_ENEMIES_ORDER.chars();
    for char in order_of_spawn_to_char {
        match char {
            '1' => {
                spawn_enemy(EnemyCharacteristics::enemy_1(&asset_server), next_enemy_postion);
                next_enemy_postion -= 75.;
            },
            _ => next_enemy_postion -= 500.,
        };

    }

} 
fn enemy_movement (
    mut query: Query<(&mut Transform), With<Enemy>>,
) {
    for (mut tf) in query.iter_mut() {
        tf.translation.x += 10. * 1. / 60.
    }
}
fn debug_enemy_health (
    mut commands: Commands,
    mut ev_change_helth: EventReader<ChangeHealthEvent>,
    mut ev_enemy_died: EventWriter<EnemyDespawned>,
    mut enemy_query:Query<(&mut EnemyCharacteristics, Entity), With<Enemy>> 
) {
    for event in ev_change_helth.iter() {
        for (mut crs, entity) in enemy_query.iter_mut() {
            let (dmg, id) = (event.0, event.1);
            if entity.id() == id {
                crs.health += dmg;
                println!("enemy got damage");
                if crs.health <=  0. {
                    commands.entity(entity).despawn();
                    ev_enemy_died.send(EnemyDespawned(entity.id()));
                }
            }
        }
    }
}