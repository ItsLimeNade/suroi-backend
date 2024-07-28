use crate::utils::ansi_coloring::consts::DATETIME_STYLE;
use crate::utils::math::consts::{HALF_PI, PI};
use crate::utils::ansi_coloring::{self, style_text, consts};
use crate::utils::random::weighted_random;
use std::collections::HashMap;
use crate::config::{self, CONFIG};
use chrono::{Local, Utc};

pub mod logger {
    /// Prints a log message to the console.
    /// ## Parameters
    /// - `message`: The messages to print as the log item
    macro_rules! console_log {
        ($($message:expr),*) => {
            {
                use crate::utils::misc::internal_log;
                internal_log(&vec![$($message),*].join(" "));
            }
        };
    }

    /// Prints a `[WARNING]` message to the console.
    /// ## Parameters
    /// - `message`: The messages to print as the log item
    macro_rules! console_warn {
        ($($message:expr),*) => {
            {
                use crate::utils::ansi_coloring::{style_text, consts::*};
                use crate::utils::misc::internal_log;
                internal_log(&format!("{} {}", &style_text("[WARNING]", &vec![WARN_STYLE]), &vec![$($message),*].join(" ")));
            }
        };
    }

    pub(crate) use {console_log, console_warn};
}

/// Internal function to print and format a log message.
/// ## Parameters
/// - `message`: The formatted messages to print
pub fn internal_log(message: &str) {
    let date = Local::now().format("[%F %T]").to_string();
    println!("{} {}",
        style_text(&date, &vec![DATETIME_STYLE]), message
    );
}

pub fn drag_const(aggressiveness: f32, base: Option<f32>) -> f32 {
    let tps = CONFIG.tps as f32;
    let a: f32 = -(aggressiveness + (1.0 / ((1.78734 * tps))).powf(2.32999)) / tps;
    if base.is_some() {
        return base.unwrap().powf(a);
    } else {
        return a.exp();
    }
}

// TODO: Implement `get_rand_ID_str` and `get_ltable_loot`
/*
pub fn get_rand_ID_str<T: ObjectDefinition>() {

}*/

/// Iterate over a list, find the first item with a given value, if exists, remove from the list.
/// ## Parameters
/// - `list`: The list to iterate over.
/// - `value`: The value to check for.
pub fn remove_from<T: PartialEq>(list: &mut Vec<T>, value: T) {
    if let Some(pos) = list.iter().position(|x| *x == value) {
        list.remove(pos);
    }
}

pub const CARDINAL_DIRECTIONS: [f64; 4] = [0.0, HALF_PI, PI, 1.5 * PI];
