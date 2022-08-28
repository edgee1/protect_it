#![allow(unused)]

use std::{iter, fs, time, thread};

use bevy::prelude::*;
use bevy_ecs_tilemap::{tiles::TileStorage, TilemapBundle};

use crate::{ENEMY_1_TEXTURE, Textures, setup, LEVEL_01_ENEMIES_ORDER, ChangeHealthEvent, EnemyDespawned, Path, RoadTiles, TILEMAP_SIZE, TILE_SIZE, START_POS};

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

#[derive(Component)]
struct CurrentCheckpoint{
    checkpoint_id: u32,
    current_x_pos: f32,
    current_y_pos: f32,
}

impl EnemyCharacteristics {
    fn enemy_1 (
        asset_server: &AssetServer,
    ) -> Self {
        EnemyCharacteristics { 
        texture: asset_server.load("enemy_1.png"), 
        size: Vec2::new(30., 30.), 
        speed: 150., 
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
                translation: Vec3::new(position, START_POS.1, 10.) , 
                ..Default::default() },
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(characteristics)
        .insert(MarkedBy(Vec::new()))
        .insert(CurrentCheckpoint{checkpoint_id: 0, current_x_pos: START_POS.0, current_y_pos: START_POS.1});
    };

    //spawning distance
    let distance_between_enemies:f32= 25.;
    let distance_between_waves:f32= 500.;

    //spawning enemy wavws 
    let mut next_enemy_postion:f32 = START_POS.0;
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
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut CurrentCheckpoint, &EnemyCharacteristics, Entity), With<Enemy>>,
) {
    let checkpoints = RoadTiles::level_01().0;
    
    for (mut tf, mut current_checkpoint, crs, entity) in query.iter_mut() {
        if current_checkpoint.checkpoint_id  < (checkpoints.len() - 2) as u32  { // check to not panic
            // calculating values
            let current_ch = current_checkpoint.checkpoint_id;
            let curr_distance = crs.speed * 1. / 60.;
            let curr_dir = if checkpoints.get(current_ch as usize + 2).unwrap() > &0 {1} else {-1};
            if current_ch % 2 /*if next check by x pos */ == 0 {
                let current_x_ch_pos = current_checkpoint.current_x_pos;
                //add to position of current checkpoint distance you need to travel until next checkpoint
                let next_ch_pos = current_x_ch_pos + *checkpoints.get(current_ch as usize + 2).unwrap() as f32 * TILE_SIZE.0;
                tf.translation.x += curr_distance * curr_dir as f32;
                // check if we are at checkpoint
                if (tf.translation.x - next_ch_pos as f32).abs() <= curr_distance {
                    current_checkpoint.checkpoint_id += 1;
                    current_checkpoint.current_x_pos = next_ch_pos;
                    continue;
                }    
            } else {
                // same for y direction
                let current_y_ch_pos = current_checkpoint.current_y_pos;
                let next_ch_pos = current_y_ch_pos + *checkpoints.get(current_ch as usize + 2).unwrap() as f32 * TILE_SIZE.1;
                tf.translation.y += curr_distance * curr_dir as f32;
                if (tf.translation.y - next_ch_pos as f32).abs() <= curr_distance {
                    current_checkpoint.checkpoint_id += 1;
                    current_checkpoint.current_y_pos = next_ch_pos;
                    continue;
                }
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
fn debug_enemy_health (
    mut commands: Commands,
    mut ev_change_helth: EventReader<ChangeHealthEvent>,
    mut ev_enemy_died: EventWriter<EnemyDespawned>,
    mut enemy_query:Query<(&mut EnemyCharacteristics, Entity), (With<Enemy>, Changed<EnemyCharacteristics>)> 
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