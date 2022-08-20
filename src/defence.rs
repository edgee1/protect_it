
use std::{time::Duration};
use bevy::{prelude::*, ecs::query::WorldQuery, utils::HashSet, reflect::erased_serde::private::serde::de};
use rand::Rng;
use crate::{Textures, WinSize, enemy::{Enemy, EnemyCharacteristics, MarkedBy}, VELOCITY, SHOOT_RADIUS, TIME_STEP, RELOAD_TIME, Events, ChangeHealthEvent, EnemyDespawned};
#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Defence;
#[derive(Component)]
struct DefenceTarget{
    distance_from_target: f32,
    target_id: u32,
}
#[derive(Component)]
struct ProjectileTarget {
    position: Vec3,
    target_id: u32,
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
                translation: Vec3::new(rand.gen_range(-win_size.w / 2. ..win_size.w / 2. ), rand.gen_range(-win_size.h / 2. ..win_size.h / 2.), 1.), 
                ..Default::default() 
                },
            sprite: Sprite { custom_size: Some(Vec2::new(50., 50.)), ..Default::default()},
            ..Default::default()
        })
        .insert(Defence)
        .insert(DefenceConfig::cannon())
        .insert(DefenceTarget{target_id: 0, distance_from_target: 0.});
    }
}

fn defence_shoot (
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut MarkedBy, &mut EnemyCharacteristics, Entity), With<Enemy>>,
    mut defence_query: Query<(&mut Transform, &mut DefenceConfig,&mut DefenceTarget,  Entity), (With<Defence>, Without<Enemy>)>,
    textures: Res<Textures>,
    mut enemy_despawned_ev: EventReader<EnemyDespawned>,
    time: Res<Time>,
) {
    // create id vector and target distance vector for each defence 
    for (tf, mut config,mut target,  defence_entity) in defence_query.iter_mut() {
        for (enemy_tf,mut enemy_marked_by, mut enemy_crs , enemy_entity) in enemy_query.iter_mut() { // iterate through each enemy and defence
            let current_enemy_distance_from_defence = ((enemy_tf.translation.x - tf.translation.x).powi(2) + (enemy_tf.translation.y - tf.translation.y).powi(2)).sqrt();

            // if there is no target and there is an enemy in a shoot radius => make this enemy a new target
            if target.target_id == 0 && current_enemy_distance_from_defence <= SHOOT_RADIUS{
                target.distance_from_target = current_enemy_distance_from_defence;
                target.target_id = enemy_entity.id();
            } 
            // if we are iterating through defence with current enemy's id and reload time has past => shoot in that enemy
            if target.target_id == enemy_entity.id() {
                target.distance_from_target = current_enemy_distance_from_defence;
                if target.distance_from_target <= SHOOT_RADIUS {
                    config.reload_time.tick(time.delta());
                    if config.reload_time.finished() {
                        commands.spawn_bundle(SpriteBundle {
                            texture: textures.ball.clone(),
                            transform: Transform { translation: tf.translation, ..Default::default() },
                            sprite: Sprite {custom_size: Some(Vec2::new(30., 30.)), ..Default::default()},
                            ..Default::default()
                        })
                        .insert(ProjectileTarget {position: enemy_tf.translation, target_id: enemy_entity.id()})
                        .insert(Projectile);
                        config.reload_time.reset();                    
                    } 
                } else { target.target_id = 0}
            }   

        }
    }
    for ev in enemy_despawned_ev.iter() {
        for (tf, mut config,mut target,  defence_entity) in defence_query.iter_mut() {
            if target.target_id == ev.0 {
                target.target_id = 0;
            }
        }
    }
}

fn projectile_movement (
    mut commands: Commands,
    mut projectile_query: Query<(&ProjectileTarget, &mut Transform, Entity), With<Projectile>>,
    mut enemy_query: Query<(&mut Transform, &mut MarkedBy, Entity), (With<Enemy>, Without<Projectile>)>,
    mut change_enemy_health_ev: EventWriter<ChangeHealthEvent>,
) {
    for (target, mut projectile_tf, projectile_entity) in projectile_query.iter_mut() {
        let projectile_position = Vec2::new(projectile_tf.translation.x, projectile_tf.translation.y);
        let target_position = Vec2::new(target.position.x, target.position.y);
        let offset = TIME_STEP * VELOCITY;
        if ((projectile_position.x - target_position.x).powi(2) + (projectile_position.y - target_position.y).powi(2)).sqrt() <= offset{                    
            change_enemy_health_ev.send(ChangeHealthEvent(-2., target.target_id));
            commands.entity(projectile_entity).despawn();

        } else {
            let direction = Vec2::new(target_position.x - projectile_position.x, target_position.y - projectile_position.y).normalize();
            let distance = TIME_STEP * VELOCITY * direction;
            projectile_tf.translation += Vec3::new(distance.x, distance.y, 0.);
        }                
    }
}

