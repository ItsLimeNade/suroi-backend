use super::utils::math::consts::*;
use std::ops::Add;

#[derive(Copy, Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Add for Orientation {
    type Output = Orientation;

    fn add(self, rhs: Self) -> Self::Output {
        let added = self as u8 + rhs as u8 % 4;
        match added {
            0 => Orientation::Up,
            1 => Orientation::Right,
            2 => Orientation::Down,
            3 => Orientation::Left,
            _ => Orientation::Up,
        }
    }
}

impl Orientation {
    pub fn to_angle(self) -> f64 {
        match self {
            Orientation::Up => 0.0,
            Orientation::Right => -HALF_PI,
            Orientation::Down => -PI,
            Orientation::Left => -HALF_PI * 3.0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Variant {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub enum GameRejectType {
    Warn,
    Temp,
    Perma,
}

pub enum GameResponse {
    Success {
        game_id: u32,
    },
    Failure {
        message: GameRejectType,
        reason: String,
        report_id: String,
    },
}

pub struct CustomTeamPlayerInfo<'a> {
    id: u32,
    is_leader: Option<bool>,
    name: &'a str,
    skin: &'a str,
    badge: Option<&'a str>,
    name_color: Option<i32>,
}

pub enum CustomTeamMessage<'a> {
    Join {
        id: u32,
        team_id: String,
        is_leader: bool,
        auto_fill: bool,
        locked: bool,
        players: &'a [CustomTeamPlayerInfo<'a>],
    },
    PlayerJoin(CustomTeamPlayerInfo<'a>),
    PlayerLeave {
        id: u32,
        new_leader_id: Option<u32>,
    },
    Settings {
        auto_fill: Option<bool>,
        locked: Option<bool>,
    },
    Start,
    Started,
}

// New stuff as of Rust below

pub struct GameConstants<'a> {
    pub protocol_version: u16,
    pub grid_size: u8,
    pub bleed_out_dpms: f32, // === 2 dps
    pub max_position: u16,
    pub player: PlayerGameConstants<'a>,
    pub loot_spawn_distance: f32,
    pub airdrop: AirdropGameConstants,
}

pub struct PlayerGameConstants<'a> {
    pub radius: f32,
    pub name_max_length: u8,
    pub default_name: &'a str,
    pub default_skin: &'a str,
    pub default_health: u8,
    pub max_adrenaline: u8,
    // inventorySlotTypings,
    // maxWeapons: inventorySlotTypings.length,
    pub kill_leader_min_kills: u8,
    pub max_mouse_dist: u8, // u8 goes to 255, change to u16 if it could be >255
    pub revive_time: u16,
    pub max_revive_dist: f32,
}

pub struct AirdropGameConstants {
    pub fall_time: u16,
    pub fly_time: u16,
    pub damage: u16,
}

pub struct GameConfig<'a> {
    pub host: &'a str,
    pub port: u16, // Port numbers only go to 65535. Right?
    pub map_name: &'a str,
    pub tps: u8, // If you want higher than 255 TPS, change this to u16.
    pub plugins: Vec<&'a str>,
    // pub spawn: { mode: SpawnMode.Normal },
    pub max_players_per_game: u8, // If you want more than 255 players per game, change this to u16.
    pub max_games: u8,
    pub prevent_join_after: u16, // If you want the value to be >65535, change this to u32.
    // pub gas: { mode: GasMode.Normal },
    pub movement_speed: f32,
    pub censor_usernames: bool,
    pub max_team_size: u8,
    // pub roles: Vec<Role>, // NOT IMPLEMENTED
}

pub struct Role { // NOT IMPLEMENTED
    name: String,
    password: String,
    is_dev: bool,
}
