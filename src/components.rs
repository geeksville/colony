use bevy::prelude::*;
use super::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Position {
    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32,
        )
    }
    pub fn to_transform_layer(&self, layer: f32) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32 + layer,
        )
    }
    pub fn distance(&self, other: &Position) -> i32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt() as i32
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    InGame,
    MainMenu,
    Paused,
}

#[derive(PartialEq)]
pub enum MenuStates { // Sorted in order of display.
    Home, Tasks, Farm, Zone, Build, Craft
}

impl MenuStates {
    pub fn to_index(&self) -> usize {
        match self { // Sorted in order of display.
            MenuStates::Home => 0,
            MenuStates::Tasks => 1,
            MenuStates::Farm => 2,
            MenuStates::Zone => 3,
            MenuStates::Build => 4,
            MenuStates::Craft => 5,
        }
    }
}


#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ActorType { // Entity? Character? Creature? Actor? Avatar? Unit? Agent?
    Dwarf,
    ManCrazy,
    Elf,
    Teen,
    Ranger,
    Woman,
    Man,
    Man2,
    ManCave,
    Pig,
    Rat,
}
impl ActorType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            ActorType::Dwarf => (59 ,13),
            ActorType::ManCrazy => (59, 15),
            ActorType::Elf => (59, 18),
            ActorType::Teen => (60, 11),
            ActorType::Ranger => (59, 22),
            ActorType::Woman => (60, 48),
            ActorType::Man => (66, 46),
            ActorType::Man2 => (60, 21),
            ActorType::ManCave => (60, 25),
            ActorType::Pig => (64, 0),
            ActorType::Rat => (64, 19),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Grass,
    Dirt,
    Gravel,
    Sand,
    Stone,
    Water,
    WallBrick,
    WallGame,
    WallMetal,
    WallStone,
    WallWood,
}

impl TileType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            TileType::Grass => (9, 11),
            TileType::Dirt => (4, 1),
            TileType::Gravel => (7, 42),
            TileType::Sand => (7, 42),
            TileType::Stone => (3, 61),
            TileType::Water => (5, 12),
            TileType::WallGame => (7, 20),
            TileType::WallStone => (7, 21),
            TileType::WallWood => (7, 22),
            TileType::WallBrick => (4, 10),
            TileType::WallMetal => (7, 24),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn is_wall(&self) -> bool {
        matches!(self, TileType::WallGame | TileType::WallStone | TileType::WallWood | TileType::WallBrick | TileType::WallMetal)

    }
}

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct MainMenuOverlay;

pub trait HoverNote {
    fn hover_note(&self) -> String;
}

#[derive(Component)]
pub struct Food {
    pub nutrition: f32,
    pub spoilage: f32,
    pub spoilage_rate: f32,
    pub name: String,
}
impl Default for Food {
    fn default() -> Self {
        Food {
            nutrition: 10.0,
            spoilage: 1.0,
            spoilage_rate: 0.03,
            name: "Food".to_string(),
        }
    }
}
impl HoverNote for Food {
    fn hover_note(&self) -> String {
        let spoilage_percent = self.spoilage * 100.0;
        format!("Spoilage: {:.2}%", spoilage_percent)
    }
}

#[derive(Component)]
pub struct HasName {
    pub name: String,
}

#[derive(Component)]
pub struct IsName;

#[derive(Component)]
pub struct HasNameShown;

#[derive(Component)]
pub struct TextName;

#[derive(Component)]
pub struct HighlightBox;

#[derive(Component)]
#[derive(Default)]
pub struct Highlighted;

#[derive(Component)]
pub struct ClickedOn;

#[derive(Component)]
pub struct InGameButton;

pub trait InfoPanel {
    fn info_panel(&self) -> Vec<String>;
}

#[derive(Component)]
pub struct Status {
    pub needs_food: Option<NeedsFood>,
    pub needs_entertainment: Option<NeedsEntertainment>,
    pub needs_sleep: Option<NeedsSleep>,
    pub index: usize,
    pub crisis: Option<String>,
    pub danger: Option<String>,
    pub injured: bool
}
impl InfoPanel for Status {
    fn info_panel(&self) -> Vec<String> {
        let mut info_lines = Vec::new();
        if let Some(needs_food) = &self.needs_food {
            info_lines.push(format!("Food: {:.2}%", needs_food.current / needs_food.max * 100.0));
        }
        if let Some(needs_entertainment) = &self.needs_entertainment {
            info_lines.push(format!("Entertainment: {:.2}%", needs_entertainment.current / needs_entertainment.max * 100.0));
        }
        if let Some(needs_sleep) = &self.needs_sleep {
            info_lines.push(format!("Sleep: {:.2}%", needs_sleep.current / needs_sleep.max * 100.0));
        }
        info_lines
    }
}


#[derive(Component, PartialEq)]
#[derive(Default)]
pub struct Brain {
    pub motivation: Option<Motivation>,
    pub task: Option<Task>,
    pub order: Option<String>,
}
impl Brain {
    pub fn remotivate(&mut self) {
        self.motivation = None;
        self.task = None;
        self.order = None;
    }
}


#[derive(Component)]
pub struct Foragable;

#[derive(Component)]
pub struct Choppable;

#[derive(Component)]
pub struct GiveMeAName;

#[derive(Component)]
pub struct Plant {
    pub growth: f32,
    pub plant_type: PlantType,
}
impl HoverNote for Plant {
    fn hover_note(&self) -> String {
        format!("{:?} Growth: {:.2}%", self.plant_type, self.growth * 100.0)
    }
}

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component)]
pub struct ZoneMarker;


#[derive(Component)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub plant_type: PlantType,
    pub material_delivered: bool,
}

impl Default for Zone {
    fn default() -> Self {
        Zone {
            zone_type: ZoneType::Farm,
            plant_type: PlantType::Cabbage,
            material_delivered: false,
        }
    }
}

pub struct NearestEntity {
    pub entity: Entity,
    pub position: Position,
    pub distance: i32,
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ZoneType {
    Farm, Pasture, Storage, Fishing, Hospital, Party, Meeting
}


#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Task { // Sorted in order of prioritization.
    Crisis, Flee, Fight, Eat, Hospital, Sleep, Sleeping, Play, Order, Work, Meander, Idle,
    Doctor, Forage, Plant, Harvest, Mine, Chop, Construct, Hunt, Milk, Cook, Fish, Craft, Clean, Haul // Forms of work
}
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Motivation { // Sorted in order of prioritization.
    Crisis, Order, Danger, Hunger, Thirst, Tired, Bored, Injured, Sick, Happy, Sad, Angry, Lonely, Love, Fear, Hate, Work, Meander, Idle
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ItemType {
    Cabbage,
    Carrot,
    CedarLog,
    PineLog,
    OakLog,
}

impl ItemType {
    pub fn sprite_index(&self) -> usize {
        match self {
            ItemType::Cabbage => 94*64+33,
            ItemType::Carrot => 94*64+24,
            ItemType::CedarLog => 94*64+30,
            ItemType::PineLog => 94*64+30,
            ItemType::OakLog => 94*64+30,
        }
    }
    pub fn nutrition(&self) -> f32 {
        match self {
            ItemType::Cabbage => 10.0,
            ItemType::Carrot => 10.0,
            _ => 0.0,
        }
    }
    // pub fn spoilage(&self) -> f32 {
    //     match self {
    //         ItemType::Cabbage => 1.0,
    //         ItemType::Carrot => 1.0,
    //         _ => 0.0,
    //     }
    // }
    pub fn spoilage_rate(&self) -> f32 {
        match self {
            ItemType::Cabbage => 0.1,
            ItemType::Carrot => 0.1,
            _ => 0.01,
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum PlantType {
    Aloe,
    Azalea,
    Bush,
    Cabbage,
    CactusRound,
    CactusUp,
    Carrot,
    CedarTree,
    FlowerBush,
    PineTree,
    OakTree,
    ThornBush,
    Vine,
    Weed,
}

impl PlantType {
    pub fn is_edible(&self) -> bool {
        matches!(self, PlantType::Cabbage)
    }
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            PlantType::Aloe => (67, 57),
            PlantType::Azalea => (67, 57),
            PlantType::Bush => (67, 57),
            PlantType::Cabbage => (94, 32),
            PlantType::CactusRound => (67, 57),
            PlantType::CactusUp => (67, 57),
            PlantType::Carrot => (94, 31),
            PlantType::CedarTree => (13, 15),
            PlantType::PineTree => (13, 13),
            PlantType::OakTree => (13, 14),
            PlantType::ThornBush => (67, 57),
            PlantType::FlowerBush => (67, 57),
            PlantType::Vine => (67, 57),
            PlantType::Weed => (67, 57),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn growth_speed(&self) -> f32 {
        match self {
            PlantType::Cabbage => 0.001,
            _ => 0.01
        }
    }
    pub fn is_forageable(&self) -> (Option<ItemType>, i32, ForageType) {
        match self {
            PlantType::Cabbage => (Some(ItemType::Cabbage), 1, ForageType::Once),
            PlantType::Carrot => (Some(ItemType::Carrot), 1, ForageType::Once),
            _ => (None, 0, ForageType::Once),
        }
    }
    pub fn is_choppable(&self) -> (Option<ItemType>, i32) {
        match self {
            PlantType::PineTree => (Some(ItemType::PineLog), 1),
            PlantType::OakTree => (Some(ItemType::OakLog), 1),
            PlantType::CedarTree => (Some(ItemType::CedarLog), 1),
            _ => (None, 0),
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ForageType {
    Once, Repeat
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum SelectableType {
    Choppable,
    Constructable,
    Foragable,
    Gatherable,
    Harvestable,
    Mineable,
    Nothing,
    Unselecting,
    Unzoning,
    Zoning,
}

#[derive(Component)]
pub struct WorkTarget;

#[derive(Component)]
pub struct Renderable {
    //pub glyph: rltk::FontCharType,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component)]
pub struct Bed;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Position>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct MoveRandom;

#[derive(Component)]
pub struct MonsterGenerator;

#[derive(Component)]
pub struct GeneratedBy {
    pub entity: Entity,
}
#[derive(Component)]
pub struct Targeting {
    pub target: Entity,
}

#[derive(Component)]
pub struct Pathing {
    pub path: Vec<Position>,
    pub destination: Position,
    pub unreachable: bool
}

impl Default for Pathing {
    fn default() -> Self {
        Pathing {
            path: Vec::new(),
            destination: Position { x: 0, y: 0, z: 0 },
            unreachable: false
        }
    }
}

#[derive(Component)]
pub struct MoveTowardsTarget;

#[derive(Component)]
pub struct MoveTowardsNearestAttackable;

#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct SizeXYZ {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl SizeXYZ {
    pub fn cube(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: x,
        }
    }
    pub fn flat(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 0.1,
        }
    }
    pub fn flat_2(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 1.0,
        }
    }
}

// NEEDS

#[derive(Component)]
pub struct NeedsFood {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}
#[derive(Component)]
pub struct NeedsEntertainment {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}
#[derive(Component)]
pub struct NeedsSleep {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}

#[derive(Component)]
#[derive(Default)]
pub struct Logs;

