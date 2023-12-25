use crate::board::Color;

impl Color {
    pub fn ansi_code(&self) -> &str {
        match *self {
            Color::Black => "\x1B[30m",
            Color::Red => "\x1B[31m",
            Color::Green => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue => "\x1B[34m",
            Color::Magenta => "\x1B[35m",
            Color::Cyan => "\x1B[36m",
            Color::White => "\x1B[37m",
            Color::AnsiReset => "\x1B[0m",
        }
    }
}
