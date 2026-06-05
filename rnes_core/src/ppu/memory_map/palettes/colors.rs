use std::{error::Error};

use crate::ppu::memory_map::palettes::lookup::{ParseError};

#[derive(Copy, Clone, Debug)]
pub struct Color {pub r: u8, pub g:u8, pub b:u8}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        return Color {r, g, b};
    }

    pub fn parse_str<'a>(s: &'a str) -> Result<Self, ParseError> {
        let values = s
            .split(" ")
            .collect::<Vec<&str>>();

        if(values.len() != 3) {
            return Err(ParseError::InvalidNumOfChannels { channels: (values.len()) })
        }

        let color_channels: Vec<u8> = values
            .iter()
            .enumerate()
            .map(|(i, &channel)| {
                match channel.parse::<u8>() {
                    Ok(v) => Ok(v),
                    Err(_) => {
                        Err(ParseError::InvalidChannelValue { 
                            channel: i, 
                            value: channel.to_string()
                        })
                    }
                }
            })
            .collect::<Result<Vec<u8>, ParseError>>()?; 

        Ok(Color {
            r: color_channels[0],
            g: color_channels[1],
            b: color_channels[2]
        })
    }
}