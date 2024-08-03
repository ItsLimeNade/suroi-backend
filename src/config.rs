use crate::typings::*;
use crate::constants::TeamSize;
use crate::typings::{SpawnMode, GasMode, MaxTeamSize};
use phf::phf_map;

pub const CONFIG: GameConfig = GameConfig {
    host: "127.0.0.1",
    port: 8000,
    ssl: None,

    map_name: "main",

    tps: 40,

    plugins: vec![],

    spawn: SpawnSettings {
        mode: SpawnMode::Normal,
        position: None,
        radius: None
    },
    
    max_players_per_game: 80,
    max_games: 4,
    prevent_join_after: 60000,

    gas: GasSettings {
        mode: GasMode::Normal,
        override_position: None,
        override_duration: None
    },

    movement_speed: 0.02655,

    censor_usernames: true,

    max_team_size: MaxTeamSize::Constant(TeamSize::Solo),

    protection: None,
    ip_header: None,
    
    roles: phf_map! {
        "developr" => Role { password: "developr", is_dev: true },
        "moderatr" => Role { password: "moderatr", is_dev: true },
        "trial_moderatr" => Role { password: "trial_moderatr", is_dev: false },
        "designr" => Role { password: "designr", is_dev: false },
        "lead_designr" => Role { password: "lead_designr", is_dev: false },
        "vip_designr" => Role { password: "vip_designr", is_dev: false },
        "studio_managr" => Role { password: "studio_managr", is_dev: false },
        "composr" => Role { password: "composr", is_dev: false },
        "lead_composr" => Role { password: "lead_composr", is_dev: false },
        "youtubr" => Role { password: "youtubr", is_dev: false },
        "boostr" => Role { password: "boostr", is_dev: false },

        "hasanger" => Role { password: "hasanger", is_dev: true },
        "leia" => Role { password: "leia", is_dev: true },
        "katie" => Role { password: "katie", is_dev: true },
        "eipi" => Role { password: "eipi", is_dev: true },
        "error" => Role { password: "error", is_dev: true },
        "kenos" => Role { password: "kenos", is_dev: true },
        "radians" => Role { password: "radians", is_dev: true },
        "limenade" => Role { password: "limenade", is_dev: true },
        "123op" => Role { password: "123op", is_dev: false }
    },
    enable_lobby_clearing: true,
    auth_server: Some(AuthServer {
        address: "http://localhost:8080"
    })
};
