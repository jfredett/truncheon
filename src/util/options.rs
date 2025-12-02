use std::sync;

use clap::{command, Parser};
use ratatui_image::picker::ProtocolType;

use crate::util::svg_template::SVGTemplate;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Options {
    // protocol: ProtocolType,
    // font_scale: (u16, u16)
}

// process Options -> Parameters
#[derive(Debug, Clone)]
pub struct Parameters {
    pub protocol_type: ProtocolType,
    pub font_size: (u16, u16),
}


impl Default for Parameters {
    fn default() -> Self {
        // TODO: "Safe" defaults (Halfblocks) instead of convenient ones
        Self {
            protocol_type: ProtocolType::Kitty,
            font_size: (8,12),
        }
    }


}
