// Thanks to @ersek-huba for remaking this file in Rust for us!

pub struct ColorData {
    pub normal: u8,
    pub bright: u8
}

pub struct Colors {
    pub black: ColorData,
    pub red: ColorData,
    pub green: ColorData,
    pub yellow: ColorData,
    pub blue: ColorData,
    pub magenta: ColorData,
    pub cyan: ColorData,
    pub white: ColorData,
    pub r#default: ColorData,
}

pub struct ColorStyles {
    pub foreground: Colors,
    pub background: Colors
}

pub struct FontStyles {
    pub bold: u8,
    pub faint: u8,
    pub italic: u8,
    pub underline: u8,
    pub blinkSlow: u8,
    pub blinkFast: u8,
    pub invert: u8,
    pub conceal: u8,
    pub strikethrough: u8,
    pub overlined: u8
}

pub const COLOR_STYLES: ColorStyles = ColorStyles {
    foreground: Colors {
        black: ColorData { normal: 30, bright: 90 },
        red: ColorData { normal: 31, bright: 91 },
        green: ColorData { normal: 32, bright: 92 },
        yellow: ColorData { normal: 33, bright: 93 },
        blue: ColorData { normal: 34, bright: 94 },
        magenta: ColorData { normal: 35, bright: 95 },
        cyan: ColorData { normal: 36, bright: 96 },
        white: ColorData { normal: 37, bright: 97 },
        r#default: ColorData { normal: 39, bright: 39 }
    },
    background: Colors {
        black: ColorData { normal: 40, bright: 100 },
        red: ColorData { normal: 41, bright: 101 },
        green: ColorData { normal: 42, bright: 102 },
        yellow: ColorData { normal: 43, bright: 103 },
        blue: ColorData { normal: 44, bright: 104 },
        magenta: ColorData { normal: 45, bright: 105 },
        cyan: ColorData { normal: 46, bright: 106 },
        white: ColorData { normal: 47, bright: 107 },
        r#default: ColorData { normal: 49, bright: 49 }
    }
};

pub const FONT_STYLES: FontStyles = FontStyles {
    bold: 1,
    faint: 2,
    italic: 3,
    underline: 4,
    blinkSlow: 5,
    blinkFast: 6,
    invert: 7,
    conceal: 8,
    strikethrough: 9,
    overlined: 53
};

pub const CSI: char = '\u{001b}';

/// Returns ANSI formatted text.
/// ## Parameters
/// - `string`: The string to be formatted
/// - `styles`: The ANSI style escape code(s) to be applied
//#[vararg]
pub fn style_text(string: &str, styles: &[u8]) -> String {
    let str_styles = styles.iter().map(|i| i.to_string() ).collect::<Vec<String>>();
    format!("{}[{}m{}{}[0m", CSI, str_styles.join(";"), string, CSI)
}

// Constants for the default styles.
// If these are faulty, don't blame @ersek-huba, he did not create these.
pub mod consts {
    use crate::utils::ansi_coloring::COLOR_STYLES;

    /// Constant for the date and time style for logs
    pub const DATETIME_STYLE: u8 = COLOR_STYLES.foreground.green.bright;

    /// Constant for the `[WARNING]` style in warnings
    pub const WARN_STYLE: u8 = COLOR_STYLES.foreground.yellow.normal;
}
