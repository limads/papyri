/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use cairo::Context;
use cairo::{ScaledFont, FontWeight, FontSlant};
use super::context_mapper::*;
use std::f64::consts::PI;
use regex::Regex;
use std::default::Default;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct FontData {
    pub font_family : String,
    pub font_weight : FontWeight,
    pub font_slant : FontSlant,
    pub font_size : i32,
    pub sf : ScaledFont
}

impl Default for FontData {

    fn default() -> Self {
        let font_family = String::from("Monospace");
        let font_weight = FontWeight::Normal;
        let font_slant = FontSlant::Normal;
        let font_size = 12;
        let sf = create_scaled_font(&font_family[..], font_slant, font_weight, font_size);
        Self {
            font_family,
            font_weight,
            font_slant,
            font_size,
            sf
        }
    }

}

impl FontData {

    pub fn create_standard_font() -> Self {
        let font_family = String::from("Liberation Sans");
        let font_weight = FontWeight::Normal;
        let font_slant = FontSlant::Normal;
        let font_size = 12;
        let sf = create_scaled_font(
            &font_family[..],
            font_slant,
            font_weight,
            font_size
        );
        Self{ font_family, font_weight, font_slant, font_size, sf }
    }

    pub fn new_from_string(font : &str) -> Self {
        let digits_pattern = Regex::new(r"\d{2}$|\d{1}$").unwrap();
        let sz_match = digits_pattern.find(&font).expect("No font size");
        let sz_txt = sz_match.as_str();
        let font_size = sz_txt.parse().expect("Unable to parse font");
        let mut prefix = &font[0..sz_match.start()];
        let slant_pattern = Regex::new("Italic|Oblique").unwrap();
        let slant_match = slant_pattern.find(prefix);
        let font_slant = match slant_match {
            Some(m) => {
                match m.as_str() {
                    "Italic" => FontSlant::Italic,
                    "Oblique" => FontSlant::Oblique,
                    _ => FontSlant::Normal
                }
            },
            None => FontSlant::Normal
        };
        if let Some(slant) = slant_match {
            prefix = &font[0..slant.start()];
        };
        let weight_pattern = Regex::new("Bold").unwrap();
        let weight_match = weight_pattern.find(prefix);
        let font_weight = match weight_match {
            Some(_w) => FontWeight::Bold,
            None => FontWeight::Normal
        };
        if let Some(weight) = weight_match {
            prefix = &font[0..weight.start()];
        };
        let font_family = String::from(prefix.trim());
        let sf = create_scaled_font(
            &font_family[..],
            font_slant,
            font_weight,
            font_size
        );
        Self {
            font_family,
            font_weight,
            font_slant,
            font_size,
            sf
        }
    }

    pub fn description(&self) -> String {
        let mut font = self.font_family.to_string();
        font = font + match self.font_slant {
            FontSlant::Normal => "",
            FontSlant::Oblique => " Oblique",
            FontSlant::Italic => " Italic",
            _ => ""
        };
        font = font + match self.font_weight {
            FontWeight::Normal => "",
            FontWeight::Bold => " Bold",
            _ => ""
        };
        font = font + &self.font_size.to_string()[..];
        font
    }

    pub fn set_font_into_context(&self, ctx : &Context) {
        ctx.set_font_size(self.font_size as f64);
        ctx.select_font_face(
            &self.font_family,
            self.font_slant,
            self.font_weight
        );
    }
}

fn create_scaled_font(
    family : &str,
    slant : FontSlant,
    weight : FontWeight,
    size : i32
) -> ScaledFont {
    let mut font_m =  cairo::Matrix::identity();
    let ctm = cairo::Matrix::identity();
    font_m.scale(size as f64, size as f64);
    let opts = cairo::FontOptions::new().unwrap();
    // context.get_font_face()
    let font_face = cairo::FontFace::toy_create(
        family,
        slant,
        weight
    ).unwrap();
    ScaledFont::new(&font_face, &font_m, &ctm, &opts).unwrap()
}

/// Draw a text with horizontal and vertical extents centered
/// at the given cooridnate.
/// The last two arguments are a proportion of the text
/// extent that shuold be used to re-position the text
/// relative to the given coordinate.
pub fn draw_label(
    sf : &ScaledFont,
    ctx : &Context,
    label : &str,
    mut pos : Coord2D,
    rotate : bool,
    center : (bool, bool),
    off_x : Option<f64>,
    off_y : Option<f64>
) -> Result<(), Box<dyn Error>> {
    ctx.save()?;
    let ext = sf.text_extents(label);
    let xadv = ext.x_advance();
    let height = ext.height();
    let half_xadv = xadv / 2.0;
    let half_height = height / 2.0;
    let x_center_off = match center.0 {
        true => (-1.0)*half_xadv,
        false => 0.0
    };
    let y_center_off = match center.1 {
        true => half_height,
        false => 0.0
    };
    let ext_off_x = off_x.unwrap_or(0.0)*xadv + x_center_off;
    let ext_off_y = off_y.unwrap_or(0.0)*height + y_center_off;
    pos.x += ext_off_x;
    pos.y += ext_off_y;
    let (glyphs, _) = sf.text_to_glyphs(pos.x, pos.y, label).unwrap();
    let radius = (pos.x.powf(2.0) + pos.y.powf(2.0)).sqrt();
    if rotate {
        // ctx.translate(-radius + height, radius);
        // Using 2*height instead of right renders the y label one label
        // character height unit away from the left border.
        ctx.translate(-radius + 2. * height, radius);
        ctx.rotate(-PI/2.0);
    }
    if let Err(e) = ctx.show_glyphs(&glyphs[..]) {
        println!("{}", e);
    }
    ctx.restore()?;
    Ok(())
}
