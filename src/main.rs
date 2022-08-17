#![allow(unused)]
mod enemy;
mod defence;

use defence::DefencePlugin;

use bevy::prelude::*;
use enemy::{EnemyPlugin};

struct ChangeHealthEvent(f32/*amount of damage */, u32/*enemy id */);
struct EnemyDespawned(u32 /* despawned enemy id */);

const ENEMY_1_TEXTURE: &str = "enemy_1.png";
const CANNON_TEXTURE_TRIAL: &str = "cannon.png";
const BALL_TEXTURE: &str = "ball.png";
const TIME_STEP: f32 = 1./60.;
const RELOAD_TIME: u64 = 2;
const SHOOT_RADIUS: f32 = 300.;
pub const VELOCITY: f32 = 500.;
pub const LEVEL_01_ENEMIES_ORDER: &str = "111111111111111";

pub struct Textures {
    enemy_1: Handle<Image>,
    cannon: Handle<Image>,
    ball: Handle<Image>,
}
struct WinSize {
    h: f32,
    w: f32
}
fn main() {
    App::new()
        .add_event::<ChangeHealthEvent>()
        .add_event::<EnemyDespawned>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(EnemyPlugin)
        .add_plugin(DefencePlugin)
        .insert_resource(WindowDescriptor {
            width: 1920.,
            height: 1080.,
            title: String::from("Protect it!"),
            ..Default::default()
        })
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(Textures {
        enemy_1: asset_server.load(ENEMY_1_TEXTURE),
        cannon: asset_server.load(CANNON_TEXTURE_TRIAL),
        ball: asset_server.load(BALL_TEXTURE),
    });
    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource( WinSize {
            h: window.height(),
            w: window.width(),
        }
    )
}