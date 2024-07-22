// Thanks to @ersek-huba for remaking this file in Rust for us!

pub struct ColorData {
    normal: u8,
    bright: u8
}

pub struct Colors {
    black: ColorData,
    red: ColorData,
    green: ColorData,
    yellow: ColorData,
    blue: ColorData,
    magenta: ColorData,
    cyan: ColorData,
    white: ColorData,
    r#default: ColorData,
}

pub struct ColorStyles {
    foreground: Colors,
    background: Colors
}

pub struct FontStyles {
    bold: u8,
    faint: u8,
    italic: u8,
    underline: u8,
    blinkSlow: u8,
    blinkFast: u8,
    invert: u8,
    conceal: u8,
    strikethrough: u8,
    overlined: u8
}

pub static color_styles: ColorStyles = ColorStyles {
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

pub static font_styles: FontStyles = FontStyles {
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

//#[vararg]
pub fn style_text(string: &str, styles: &Vec<u8>) -> String {
    let str_styles = styles.iter().map(|i| i.to_string() ).collect::<Vec<String>>();
    format!("{}[{}m{}{}[0m", CSI, str_styles.join(";"), string, CSI)
}
