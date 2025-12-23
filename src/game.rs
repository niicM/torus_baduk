use std::time::Duration;
use std::collections::HashMap;
use bevy::prelude::Entity;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::loading::TextureAssets;

pub const BOARD_SIZE: usize = 13;
pub const VISIBLE_TILES: i32 = 17;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

#[derive(Resource)]
pub struct Board {
    pub array: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            array: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}


#[derive(Component)]
pub struct TileGrid {
    pub tiles: HashMap<(i32, i32), Entity>,
}

impl TileGrid {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}

pub struct GridTilePlugin;

impl Plugin for GridTilePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Board::new())
        .add_systems(
            Startup,
            |mut commands: Commands| { commands.spawn(TileGrid::new()); },
        ).add_systems(
            Update, 
            spawn_visible_tiles.run_if(on_timer(Duration::from_secs_f32(1.0))),
        );
    }
}

pub fn spawn_visible_tiles(
    mut commands: Commands,
    mut grid: Query<&mut TileGrid>,
    camera_transform: Query<&GlobalTransform, With<Camera>>,
    window: Query<&Window>,
    textures: Res<TextureAssets>,
) {
    let camera_transform = camera_transform.single().unwrap();
    let window = window.single().unwrap();
    let mut grid = grid.single_mut().unwrap();

    let camera_pos = camera_transform.translation().truncate();

    // --- Compute tile size dynamically ---
    let tile_size = (window.width().min(window.height())) / VISIBLE_TILES as f32;

    let half_tiles = VISIBLE_TILES / 2;

    let min_tile_x = -half_tiles;
    let max_tile_x = half_tiles;
    let min_tile_y = -half_tiles;
    let max_tile_y = half_tiles;

    for y in min_tile_y..=max_tile_y {
        for x in min_tile_x..=max_tile_x {
            let grid_x = x + BOARD_SIZE as i32 / 2;
            let grid_y = y + BOARD_SIZE as i32 / 2;

            let ux = grid_x as usize;
            let uy = grid_y as usize;

            if ux >= BOARD_SIZE || uy >= BOARD_SIZE {
                continue;
            }

            // Already spawned?
            if grid.tiles.contains_key(&(grid_x, grid_y)) {
                continue;
            }

            let world_x = x as f32 * tile_size;
            let world_y = y as f32 * tile_size;

            let entity = commands.spawn((
                Sprite {
                    image: textures.e.clone(),
                    custom_size: Some(Vec2::splat(tile_size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
            )).id();

            grid.tiles.insert((grid_x, grid_y), entity);
        }
    }
}
