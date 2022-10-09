/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

// use libxml::tree::node::Node;
use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
use super::*;
use super::super::MappingProperty;
use std::borrow::Borrow;

/// Represents ordered, regularly-spaced bars. The only map required by the bars
/// is their height (or width if they are horizontally spaced), since the bar position
/// is fully determined by its order in the data array, the informed offset and bar spacing (at data scale).
#[derive(Debug, Clone)]
pub struct BarMapping {
    color : RGBA,
    center_anchor : bool,

    // x and y hold the coordinates of the bar base, increasing at a fixed
    // spacing in the data space from the origin.
    x : Vec<f64>,
    y : Vec<f64>,

    // height and width hold the coordinates of the bar width and height in
    // data space. If horizontal is true, w varies with the data and h is fixed;
    // if horizontal is false, h varies wit the data and w is fixed.
    h : Vec<f64>,
    w : Vec<f64>,

    col_names : [String; 4],

    // TODO rename to bar thickness, since the graph mihgt be horizontal. This is at the scale of 1-100
    // where 100 represents a bar with thickness covering the whole bar_spacing interval.
    bar_width : f64,

    // Bar origin, at data scale.
    origin : (f64, f64),

    // Value, in data coordiantes, that positions the ith bar relative to the origin.
    // If center_anchor = false, the top-left position of the bar at index i will be at origin + bar_spacing * i
    bar_spacing : f64,

    horizontal : bool,
    source : String
}

impl Default for BarMapping {

    fn default() -> Self {
        let mut bar = Self {
            color : RGBA::BLACK,
            x : Vec::new(),
            y : Vec::new(),
            h : Vec::new(),
            w : Vec::new(),
            col_names : [String::new(), String::new(), String::new(), String::new()],
            bar_width : 100.0,
            origin : (0.0, 0.0),
            bar_spacing : 1.0,
            horizontal : false,
            center_anchor : false,
            source : String::new()
        };
        bar.adjust_bar();
        bar
    }

}

impl BarMapping {

    pub fn color(mut self, color : String) -> Self {
        self.color = color.parse().unwrap();
        self.adjust_bar();
        self
    }

    pub fn center_anchor(mut self, center_anchor : bool) -> Self {
        self.center_anchor = center_anchor;
        self.adjust_bar();
        self
    }

    pub fn width(mut self, w : f64) -> Self {
        self.bar_width = w;
        self.adjust_bar();
        self
    }

    pub fn origin(mut self, origin : (f64, f64)) -> Self {
        self.origin = origin;
        self.adjust_bar();
        self
    }

    pub fn bar_spacing(mut self, bar_spacing : f64) -> Self {
        self.bar_spacing = bar_spacing;
        self.adjust_bar();
        self
    }

    pub fn horizontal(mut self, horizontal : bool) -> Self {
        self.horizontal = horizontal;
        self.adjust_bar();
        self
    }

    pub fn map<D>(ext : impl IntoIterator<Item=D>) -> Self
    where
        D : Borrow<f64>
    {
        let mut bar : BarMapping = Default::default();
        let extension : Vec<_> = ext.into_iter().map(|d| *d.borrow() ).collect();
        bar.update_data(vec![extension]);
        bar
    }

    fn adjust_bar(&mut self) {

        /* At this point, either the w or h vectors have been set from the single
        mapping x, depending on whether horizontal is set to true, and the remaining
        data points (x, y, h||w) need to be updated. */

        if self.horizontal {
            let n = self.w.len();

            // y (bar base) increasing from origin
            self.y = (0..n).map(|i| self.origin.1 + self.bar_spacing * i as f64 ).collect();

            // x (bar base) fixed at origin
            self.x = (0..n).map(|_| self.origin.0 ).collect();

            // Decrease half spacing from y coordinate.
            if self.center_anchor {
                let spacing = self.bar_spacing;
                self.y.iter_mut().for_each(|y| *y -= spacing / 2. );
            }

        } else {
            let n = self.h.len();

            // y (bar base) fixed at origin
            self.y = (0..n).map(|_| self.origin.1 ).collect();

            // x (bar base) increasing from origin
            self.x = (0..n).map(|i| self.origin.0  + self.bar_spacing * i as f64 ).collect();

            // Decrease half spacing from x coordinate.
            if self.center_anchor {
                let spacing = self.bar_spacing;
                self.x.iter_mut().for_each(|x| *x -= spacing / 2. );
            }
        }

        // Update the other dimension, which should remain fixed.
        if self.horizontal {
            self.h = (0..self.w.len()).map(|_| self.bar_spacing * self.bar_width ).collect();
        } else {
            self.w = (0..self.h.len()).map(|_| self.bar_spacing * self.bar_width ).collect();
        }
    }

}

impl Mapping for BarMapping {

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(_line) => {
                unimplemented!()
            },
            _ => false
        }
    }

    fn update_from_json(&mut self, rep : crate::model::Mapping) {

        if let Some(w) = rep.width {
            self.bar_width = w;
        }

        if let Some(v) = rep.vertical {
            self.horizontal = !v;
        }

        if let Some(s) = rep.spacing {
            self.bar_spacing = s;
        }

        if let Some(origin) = rep.origin {
            if self.horizontal {
                self.origin = (0., origin);
            } else {
                self.origin = (origin, 0.);
            }
        }

        if let Some(c) = rep.center {
            self.center_anchor = c;
        }

        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }

        let mut new_data = Vec::new();
        super::update_single_data_from_json(&mut new_data, rep);
        self.update_data(vec![new_data]);
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
        ctx.set_source_rgb(self.color.red().into(), self.color.green().into(), self.color.blue().into());
        let r_iter = self.x.iter().zip(self.y.iter()
            .zip(self.w.iter()
            .zip(self.h.iter()))
        );
        for (x, (y, (w, h))) in r_iter {
            let tl_ok = mapper.check_bounds(*x, y + h);
            let tr_ok = mapper.check_bounds(x + w, y + h);
            let bl_ok = mapper.check_bounds(*x, *y);
            let br_ok = mapper.check_bounds(x + *w, *y);
            if  tl_ok && tr_ok && bl_ok && br_ok {
                let bottom_left = mapper.map(*x, *y);
                let bottom_right = mapper.map(x + *w, *y);
                let top_left = mapper.map(*x, *y + *h);
                let coord_w = bottom_left.distance(bottom_right);
                let coord_h = bottom_left.distance(top_left);
                ctx.rectangle(top_left.x, top_left.y, coord_w, coord_h);
                ctx.fill()?;
                ctx.stroke()?;
            } else {
                // println!("Out of bounds mapping");
            }
        }
        ctx.restore()?;
        Ok(())
    }

    fn update_data(&mut self, mut values : Vec<Vec<f64>>) {
        //println!("Received for updating: {:?}", values);
        /*self.x = values[0].clone();
        self.y = values[1].clone();
        self.w = values[2].clone();
        self.h = values[3].clone();*/
        if self.horizontal {
            self.w = values.remove(0);
        } else {
            self.h = values.remove(0);
        }
        assert!(values.len() == 0);
        self.adjust_bar();
    }

    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {

    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let mut xmin = self.origin.0;
        let mut ymin = self.origin.1;
        if self.center_anchor && self.horizontal {
            ymin -= self.bar_spacing / 2.0
        }
        if self.center_anchor && !self.horizontal {
            xmin -= self.bar_spacing / 2.0
        }
        let xmax = if self.horizontal {
            let max_w = self.w.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            xmin + *max_w
        } else {
            let n = self.x.len();
            xmin + n as f64 * self.bar_spacing
        };
        let ymax = if self.horizontal {
            let n = self.y.len();
            ymin + n as f64 * self.bar_spacing
        } else {
            let max_h = self.h.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            ymin + *max_h
        };
        Some(((xmin, xmax), (ymin, ymax)))
    }

    fn mapping_type(&self) -> String {
        "bar".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            "width" => self.col_names[2].clone(),
            "height" => self.col_names[3].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String, String)> {
        vec![
            (String::from("x"), self.get_col_name("x")),
            (String::from("y"), self.get_col_name("y")),
            (String::from("width"), self.get_col_name("width")),
            (String::from("height"), self.get_col_name("height"))
        ]
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols.insert("width".into(), self.col_names[2].clone());
        cols.insert("height".into(), self.col_names[3].clone());
        cols
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            "width" => { self.col_names[2] = name.into(); },
            "height" => { self.col_names[3] = name.into(); },
            _ => { }
        }
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        if cols.len() != 1 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("height", &cols[0]);
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
