use std::fmt::{self, Display, Write};
use std::path::Path;
use std::{error::Error, fs};

use crate::ppu::memory_map::palettes::colors::Color;

#[derive(Debug)]
pub struct ColorLUT {
    colors: [Color; 64]
}

#[derive(Debug)]
pub enum ParseError {
    InvalidNumOfChannels { channels: usize },
    InvalidChannelValue { channel: usize, value: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumOfChannels { channels} => {
                write!(f, "Invalid number of channels ({channels})")
            }
            ParseError::InvalidChannelValue { channel, value } => {
                write!(f, "Invalid value {value} of channel {channel}")
            }
        }
    }
}

impl<'a> Error for ParseError {}

impl ColorLUT {
    pub fn new() -> Self {
        ColorLUT {colors: [Color::new(0, 0, 0); 64]}
    }

    pub fn from_path(p: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string((p))?;
        let colors = &content
            .lines()
            .collect::<Vec<&str>>()
            [3..];

        let mut lut = ColorLUT::new();

        for (i, &color) in colors.iter().enumerate() {
            lut.colors[i] = Color::parse_str(color)?
        }

        Ok(lut)
    }
}

impl fmt::Display for ColorLUT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in [0, 2] {
            for j in 0..16 {
                let top = self.colors[j + 16 * i];
                let bottom = self.colors[j + 16 * (i + 1)];
                write!(f, "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀", top.r, top.g, top.b, bottom.r, bottom.g, bottom.b)?;
            }
            if(i != 2) {
                write!(f, "\n")?;
            }
        }
        write!(f, "\x1b[0m")?;

        Ok(())
    }
}