use crate::typings::*;
use crate::constants::TeamSize;

pub enum SpawnMode {
    Normal,
    Radius,
    Fixed,
    Center,
}

pub enum GasMode {
    Normal,
    Debug,
    Disabled,
}

pub const CONFIG: GameConfig = GameConfig {
    host: "127.0.0.1",
    port: 8000,

    map_name: "main",

    tps: 40,

    plugins: vec![],

    // IMPLEMENT THIS IF YOU KNOW HOW
    //spawn: { mode: SpawnMode.Normal },

    max_players_per_game: 80,
    max_games: 4,
    prevent_join_after: 60000,

    // IMPLEMENT THIS IF YOU KNOW HOW
    // gas: { mode: GasMode.Normal },

    movement_speed: 0.02655,

    censor_usernames: true,

    max_team_size: TeamSize::Solo as u8,

    // IMPLEMENT THIS IF YOU KNOW HOW
    /*
    roles: {
        "developr": { password: "developr", isDev: true },
        "moderatr": { password: "moderatr", isDev: true },
        "trial_moderatr": { password: "trial_moderatr" },
        "designr": { password: "designr" },
        "lead_designr": { password: "lead_designr" },
        "vip_designr": { password: "vip_designr" },
        "studio_managr": { password: "studio_managr" },
        "composr": { password: "composr" },
        "lead_composr": { password: "lead_composr" },
        "youtubr": { password: "youtubr" },
        "boostr": { password: "boostr" },

        "hasanger": { password: "hasanger", isDev: true },
        "leia": { password: "leia", isDev: true },
        "katie": { password: "katie", isDev: true },
        "eipi": { password: "eipi", isDev: true },
        "error": { password: "error", isDev: true },
        "kenos": { password: "kenos", isDev: true },
        "radians": { password: "radians", isDev: true },
        "limenade": { password: "limenade", isDev: true },
        "123op": { password: "123op" }
    },

    // IMPLEMENT THIS IF YOU KNOW HOW
    auth_server: {
        address: "http://localhost:8080"
    }
    */

    // IMPLEMENT THIS IF YOU KNOW HOW
    // roles:vec![],
};
