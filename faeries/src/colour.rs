use std::fmt;

#[allow(dead_code)]
pub enum Colour {
    Clear,
    // Foreground
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Foreground bright
    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,
    // Background
    BlackBackground,
    RedBackground,
    GreenBackground,
    YellowBackground,
    BlueBackground,
    MagentaBackground,
    CyanBackground,
    WhiteBackground,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::Clear => write!(f, "\x1b[0m"),
            // Foreground
            Colour::Black => write!(f, "\x1b[30m"),
            Colour::Red => write!(f, "\x1b[31m"),
            Colour::Green => write!(f, "\x1b[32m"),
            Colour::Yellow => write!(f, "\x1b[33m"),
            Colour::Blue => write!(f, "\x1b[34m"),
            Colour::Magenta => write!(f, "\x1b[35m"),
            Colour::Cyan => write!(f, "\x1b[36m"),
            Colour::White => write!(f, "\x1b[37m"),
            // Foreground bright
            Colour::BlackBright => write!(f, "\x1b[1;30m"),
            Colour::RedBright => write!(f, "\x1b[1;31m"),
            Colour::GreenBright => write!(f, "\x1b[1;32m"),
            Colour::YellowBright => write!(f, "\x1b[1;33m"),
            Colour::BlueBright => write!(f, "\x1b[1;34m"),
            Colour::MagentaBright => write!(f, "\x1b[1;35m"),
            Colour::CyanBright => write!(f, "\x1b[1;36m"),
            Colour::WhiteBright => write!(f, "\x1b[1;37m"),
            // Background
            Colour::BlackBackground => write!(f, "\x1b[40m"),
            Colour::RedBackground => write!(f, "\x1b[41m"),
            Colour::GreenBackground => write!(f, "\x1b[42m"),
            Colour::YellowBackground => write!(f, "\x1b[43m"),
            Colour::BlueBackground => write!(f, "\x1b[44m"),
            Colour::MagentaBackground => write!(f, "\x1b[45m"),
            Colour::CyanBackground => write!(f, "\x1b[46m"),
            Colour::WhiteBackground => write!(f, "\x1b[47m"),
        }
    }
}
