use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;

use crate::RoadTiles;

pub struct TilemapDrawPlugin;
impl Plugin for TilemapDrawPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup)
            .add_startup_system(draw_road);
    }
}
fn startup(
    mut commands: Commands, asset_server: Res<AssetServer>
) {

    let texture_handle: Handle<Image> = asset_server.load("grass_texture.png");

    let tilemap_size = TilemapSize {x: 64, y: 36};

    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);

    for x in 0..64u32 {
        for y in 0..36u32 {
            let tile_pos = TilePos {x, y};
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()  
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize {x: 32., y: 32.};

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16., y: 16. },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.,
            ),
            ..Default::default()
        });
}

fn draw_road(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_query: Query<&mut TilemapTexture>,
    mut tilemap_query: Query<&TileStorage>
) {
    let road_texture_handle: Handle<Image> = asset_server.load("road_texture_straight.png");

    let tilemap_size = TilemapSize {x: 64, y: 36};

    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);

    let road_tiles = RoadTiles::level_01().0;
    let mut x_pos_tile = road_tiles[0];
    let mut y_pos_tile = road_tiles[1];
    let mut current_dir_is_x = true;

    for (mut pos, e) in road_tiles.iter().enumerate() {        
        if pos < road_tiles.len() - 2 {
            if current_dir_is_x {
                let dir = if road_tiles[pos + 2] > 0 {1} else {-1};
                for x in 0..(road_tiles[pos + 2]  - road_tiles[pos]).abs() {
                    let tile_pos = TilePos {x: x_pos_tile as u32, y: y_pos_tile as u32};
                    let tile_entity = commands
                        .spawn()
                        .insert_bundle(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, Some(tile_entity));
                    x_pos_tile += dir;

                    println!("x pos: {}", x_pos_tile)
                }                    
                pos += 1;
                current_dir_is_x = false;
            } else {
                let dir = if road_tiles[pos + 2] > 0 {1} else {-1};
                for y in 0..(road_tiles[pos + 2].abs()  - road_tiles[pos].abs()).abs() {
                    let tile_pos = TilePos {x: x_pos_tile as u32, y: y_pos_tile as u32};
                    let tile_entity = commands
                        .spawn()
                        .insert_bundle(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, Some(tile_entity));
                    y_pos_tile += dir;
                    println!("y pos: {}", y_pos_tile)
                }
                current_dir_is_x = true;
                pos += 1;
            }
        }

    }

    let tile_size = TilemapTileSize {x: 64., y: 64.};

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16., y: 16. },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(road_texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                1.,
            ),
            ..Default::default()
        });

}