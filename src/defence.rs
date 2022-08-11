use std::{time::Duration, iter::repeat};

use bevy::{prelude::*, ecs::query::WorldQuery, utils::HashSet, reflect::erased_serde::private::serde::de};
use rand::Rng;

use crate::{Textures, WinSize, enemy::{Enemy, EnemyCharacteristics}, VELOCITY, SHOOT_RADIUS, TIME_STEP, RELOAD_TIME};
#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Defence;
#[derive(Component)]
struct Target{
    target_tf: Transform,
    target_entity: Entity,
}
pub struct DefencePlugin;
impl Plugin for DefencePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_defence)
            .add_system(defence_shoot)
            .add_system(projectile_movement);
    }
}
#[derive(Component)]
struct DefenceConfig {
    damage: f32,
    reload_time: Timer,
}
impl DefenceConfig {
    fn cannon() -> Self {
        DefenceConfig { damage: 3., reload_time: Timer::new(Duration::from_secs(RELOAD_TIME ), true) }
    }
}

fn spawn_defence(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    win_size: Res<WinSize>
) {
    let mut rand = rand::thread_rng();

    if kb.just_pressed(KeyCode::Space) {
        commands.spawn_bundle(SpriteBundle {
            texture: textures.cannon.clone(),
            transform: Transform { 
                translation: Vec3::new(rand.gen_range(-win_size.w / 2. ..win_size.w / 2. ), rand.gen_range(-win_size.h / 2. ..win_size.h / 2.), 0.), 
                ..Default::default() 
                },
            sprite: Sprite { custom_size: Some(Vec2::new(50., 50.)), ..Default::default()},
            ..Default::default()
        })
        .insert(Defence)
        .insert(DefenceConfig::cannon());
    }
}

fn defence_shoot (
    mut commands: Commands,
    textures: Res<Textures>,
    mut enemy_query: Query<(&mut Transform, Entity), With<Enemy>>,
    mut defence_query: Query<(&mut Transform, &mut DefenceConfig), (With<Defence>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (tf, mut config) in defence_query.iter_mut() {
        for (enemy_tf, enemy_entity) in enemy_query.iter_mut() {
            if ((enemy_tf.translation.x - tf.translation.x).powi(2) + (enemy_tf.translation.y - tf.translation.y).powi(2)).sqrt() <= SHOOT_RADIUS {
                config.reload_time.tick(time.delta());
                if config.reload_time.finished() {
                    commands.spawn_bundle(SpriteBundle {
                        texture: textures.ball.clone(),
                        transform: Transform { translation: tf.translation, ..Default::default() },
                        sprite: Sprite {custom_size: Some(Vec2::new(30., 30.)), ..Default::default()},
                        ..Default::default()
                    })
                    .insert(Target {target_tf: *enemy_tf, target_entity: enemy_entity})
                    .insert(Projectile);
                    config.reload_time.reset();
                }
            }
        }
    }
}

fn projectile_movement (
    mut commands: Commands,
    mut query: Query<(&Target, &mut Transform, Entity), With<Projectile>>
) {
    for (target, mut tf, entity) in query.iter_mut() {
        let target_tf = Some(target.target_tf);
        let target_entity = Some(target.target_entity);

        if target_tf.is_some() && target_entity.is_some() {
            let (target_tf, target_entity) = (target_tf.unwrap(), target_entity.unwrap());

            let projectile_position = Vec2::new(tf.translation.x, tf.translation.y);
            let target_position = Vec2::new(target_tf.translation.x, target_tf.translation.y);
            let offset = TIME_STEP * VELOCITY;

            if ((projectile_position.x - target_position.x).powi(2) + (projectile_position.y - target_position.y).powi(2)).sqrt() <= offset{

                commands.entity(entity).despawn();
                commands.entity(target_entity).despawn();
                
            } else {
                let direction = Vec2::new(target_position.x - projectile_position.x, target_position.y - projectile_position.y).normalize();
                let distance = TIME_STEP * VELOCITY * direction;
                tf.translation += Vec3::new(distance.x, distance.y, 0.);
            }
        }

    }
}