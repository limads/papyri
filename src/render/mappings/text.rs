/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
use super::text::{FontData, draw_label};
use super::*;
use std::cmp::*;
use super::super::{MappingProperty, TextProperty};
use std::fmt::Display;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct TextMapping {
    x : Vec<f64>,
    y : Vec<f64>,
    text : Vec<String>,
    font : FontData,
    color : RGBA,
    col_names : [String; 3],
    source : String
}

impl Default for TextMapping {

    fn default() -> Self {
        Self {
            color : RGBA::BLACK,
            x : Vec::new(),
            y : Vec::new(),
            text : Vec::new(),
            font : Default::default(),
            col_names : [String::new(), String::new(), String::new()],
            source : String::new()
        }
    }

}

impl TextMapping {

    pub fn color(mut self, color : String) -> Self {
        self.color = color.parse().unwrap();
        self
    }

    pub fn font(mut self, font : String) -> Self {
        self.font = FontData::new_from_string(&font);
        self
    }

    pub fn map<D, T>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>, text : impl IntoIterator<Item=T>) -> Self
    where
        D : Borrow<f64>,
        T : Display
    {
        let mut text_m : TextMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.borrow() ).collect();
        let y : Vec<_> = y.into_iter().map(|d| *d.borrow() ).collect();
        let text : Vec<_> = text.into_iter().map(|t| format!("{}", t) ).collect();
        text_m.update_data(vec![x, y]);
        text_m.set_text_data(&text);
        text_m
    }

    // Calls to update_data(.) can call this
    // function to update the text before the
    // call to draw(.).
    pub fn set_text_data(&mut self, text : &Vec<String>) {
        self.text = text.clone();
    }
}

impl Mapping for TextMapping {

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Text(text) => {
                match text {
                    TextProperty::X(x) => self.x = x,
                    TextProperty::Y(y) => self.y = y,
                    TextProperty::Text(t) => self.text = t,
                    TextProperty::Color(color) => self.color = color.parse().unwrap(),
                    TextProperty::Font(f) => self.font = FontData::new_from_string(&f),
                }
                true
            },
            _ => false
        }
    }

    fn update_from_json(&mut self, rep : crate::model::Mapping) {
        if let Some(font) = &rep.font {
            self.font = FontData::new_from_string(&font);
        }
        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }

        // println!("Mapping json rep: {:?}", rep);

        super::update_textual_data_from_json(&mut self.x, &mut self.y, &mut self.text, rep);
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
        if !((self.x.len() == self.y.len()) && (self.x.len() == self.text.len())) {
            // eprintln!("Invalid dimensions at textual mapping");
        }
        ctx.set_source_rgb(
            self.color.red().into(),
            self.color.green().into(),
            self.color.blue().into()
        );
        self.font.set_font_into_context(&ctx);
        for ((x, y), t) in self.x.iter().zip(self.y.iter()).zip(self.text.iter()) {
            if mapper.check_bounds(*x, *y) {
                let pos = mapper.map(*x, *y);
                draw_label(
                    &self.font.sf,
                    ctx,
                    t,
                    pos,
                    false,
                    (true, true),
                    None,
                    None
                )?;
            } else {
                // eprintln!("Out of bounds mapping");
            }
        }
        ctx.restore()?;
        Ok(())
    }

    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.y = values[1].clone();
    }

    fn update_extra_data(&mut self, values : Vec<Vec<String>>) {
        if let Some(tv) = values.get(0) {
            self.text = tv.clone();
        } else {
            // println!("Text data vector is empty");
        }
    }

    fn mapping_type(&self) -> String {
        "text".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            "text" => self.col_names[2].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String,String)> {
        vec![
            (String::from("x"), self.get_col_name("x")),
            (String::from("y"), self.get_col_name("y")),
            (String::from("text"), self.get_col_name("text"))
        ]
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            "text" => { self.col_names[2] = name.into(); },
            _ => { }
        }
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols.insert("text".into(), self.col_names[2].clone());
        cols
    }

    // TODO account for the full text box here, not only the top-left position.
    // Consider the special case of a single data point (not only for text,
    // but for all non-line mappings) where a minimum scale region should be
    // guaranteed to plot anything (even the grid lines) when adjust is set to tight.
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {

        // let (glyphs, _) = sf.text_to_glyphs(pos.x, pos.y, label)
        // sf.glyph_extents(&glyphs[..]);
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymin = self.y.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymax = self.y.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        if cols.len() != 3 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            self.set_col_name("text", &cols[2]);
            Ok(())
        }
    }

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}

