/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use gdk4::RGBA;
use std::error::Error;
use super::text;
use std::collections::HashMap;
use std::str::FromStr;
use crate::render::text::FontData;
use crate::model::DesignError;

#[derive(Clone, Debug)]
pub struct PlotDesign {
    pub bg_color : RGBA,
    pub grid_color : RGBA,
    pub grid_width : i32,
    pub font : text::FontData
}

impl Default for PlotDesign {

    fn default() -> Self {
        Self {
            bg_color : RGBA::from_str("#ffffff").unwrap(),
            grid_color : RGBA::from_str("#d3d7cf").unwrap(),
            grid_width : 1,
            font : FontData::new_from_string("Monospace Regular 22")
        }
    }

}

impl PlotDesign {

    pub fn new_from_json(rep : crate::model::Design) -> Result<Self, Box<dyn Error>> {
        rep.validate()?;
        let bg_color = rep.bgcolor.parse().or(Err(DesignError::InvalidBackgroundColor))?;
        let grid_color = rep.fgcolor.parse().or(Err(DesignError::InvalidGridColor))?;
        let design = Self {
            bg_color,
            grid_color,
            grid_width : rep.width,
            font : text::FontData::new_from_string(&rep.font)
        };
        Ok(design)
    }

    pub fn description(&self) -> HashMap<String, String> {
        let mut desc = HashMap::new();
        desc.insert("bg_color".into(), self.bg_color.to_string());
        desc.insert("grid_color".into(), self.grid_color.to_string());
        desc.insert("grid_width".into(), self.grid_width.to_string());
        desc.insert("font".into(), self.font.description());
        desc
    }

}

