use bevy::prelude::*;
use super::prelude::*;

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>
}

pub fn generate_map(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    biome: Res<Biome>,
) {
    let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_LENGTH {
            let tyle_type = if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_LENGTH - 1 {
                TileType::WallGame
            } else {
                //Generate random number from 0 to biome.tiles.len()
                //let random_index = rand::thread_rng().gen_range(0..biome.tiles.len());
                biome.tiles.choose(&mut rand::thread_rng()).unwrap().clone()//.unwrap_or(TileType::Grass)
                //TileType::Grass
            };
            spawn_tile(&mut commands, Position { x, y, z: 0 }, tyle_type.clone(), &sprite_sheet);
            tiletypes.insert( Position { x, y, z: 0 }, tyle_type);
        }
    }
    commands.insert_resource(TileHash { hash: tiletypes });
}
pub fn _update_map_tiles(
    mut commands: Commands,
    tiles: Query<(&Position, &TileType), With<MapTile>>,
) {
    let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
    for (tile_position, tile_type) in tiles.iter() {
        tiletypes.insert(*tile_position, tile_type.clone());
    }
    println!("tiletypes: {:?}", tiletypes);
    commands.insert_resource(TileHash { hash: tiletypes });
}

fn spawn_tile(
    commands: &mut Commands,
    position: Position,
    tile_type: TileType,
    sprite_sheet: &SpriteSheet,
) {
    let sprite =  TextureAtlasSprite::new(tile_type.sprite_index());
    // sprite.color = Color::rgb(0.5, 0.5, 0.5);
    // sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));//Some(Vec2::splat(1.0));
        // commands.spawn(SpriteBundle {
        //     sprite: Sprite {
        //         color: match tile_type { TileType::Wall => Color::GRAY, _ => Color::GREEN },
        //         custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        //         ..default()
        //     },
        //     ..default()
        // })
    commands.spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform::from_xyz(
                position.x as f32 * TILE_SIZE,
                position.y as f32 * TILE_SIZE,
                position.z as f32 * TILE_SIZE,
            ),
            ..Default::default()
        })
        .insert(MapTile)
        .insert(position)
        .insert(tile_type)
        .insert(SizeXYZ::flat(TILE_SIZE))
        .insert(position.to_transform())
        ;
}