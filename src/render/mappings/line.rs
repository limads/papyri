/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
use super::*;
use std::cmp::*;
use std::default::Default;
use super::super::{MappingProperty, LineProperty};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct LineMapping {
    color : RGBA,
    x : Vec<f64>,
    y : Vec<f64>,
    width : f64,
    dash_n : i32,
    col_names : [String; 2],
    source : String
}

impl Default for LineMapping {

    fn default() -> Self {
        Self {
            color : RGBA::BLACK,
            x : Vec::new(),
            y : Vec::new(),
            width : 1.0,
            dash_n : 1,
            col_names : [String::new(), String::new()],
            source : String::new()
        }
    }

}

impl LineMapping {

    pub fn width(mut self, w : f64) -> Self {
        self.width = w;
        self
    }

    pub fn color(mut self, color : String) -> Self {
        self.color = color.parse().unwrap();
        self
    }

    pub fn dash_n(mut self, dash_n : i32) -> Self {
        self.dash_n = dash_n;
        self
    }

    // TODO rename to data.
    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>) -> Self
    where
        D : Borrow<f64>
    {
        let mut line : LineMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.borrow() ).collect();
        let y : Vec<_> = y.into_iter().map(|d| *d.borrow() ).collect();
        line.update_data(vec![x, y]);
        line
    }

    fn build_dash(n : i32) -> Vec<f64> {
        let dash_sz = 10.0 / (n as f64);
        let mut dashes = Vec::<f64>::new();
        for _i in 1..n {
            dashes.push(dash_sz);
        }
        dashes
    }

}

impl Mapping for LineMapping {

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(line) => {
                match line {
                    LineProperty::Color(col) => { self.color = col.parse().unwrap() },
                    LineProperty::Width(w) => { self.width = w },
                    LineProperty::Dash(d) => { self.dash_n = d },
                    LineProperty::X(x) => { self.x = x },
                    LineProperty::Y(y) => { self.y = y }
                }
                true
            },
            _ => false
        }
    }

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> {
        //println!("{:?}", self);
        if self.x.len() < 2 || self.y.len() < 2 {
            return Ok(());
        }
        ctx.save()?;
        ctx.set_source_rgb(
            self.color.red().into(),
            self.color.green().into(),
            self.color.blue().into()
        );
        ctx.set_line_width(self.width);
        let dashes = LineMapping::build_dash(self.dash_n);
        ctx.set_dash(&dashes[..], 0.0);
        let zip_xy = self.x[1..].iter().zip(self.y[1..].iter());
        let from = mapper.map(self.x[0], self.y[0]);
        ctx.move_to(from.x, from.y);

        for (curr_x, curr_y) in zip_xy {
            if mapper.check_bounds(*curr_x, *curr_y) {
                let to = mapper.map(*curr_x, *curr_y);
                ctx.line_to(to.x, to.y);
            } else {
                // eprintln!("Out of bounds mapping");
            }
        }
        ctx.stroke()?;
        ctx.restore()?;
        Ok(())
    }

    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.y = values[1].clone();
    }

    fn update_from_json(&mut self, rep : crate::model::Mapping) {
        if let Some(width) = rep.width {
            self.width = width;
        }
        if let Some(dash_n) = rep.spacing {
            self.dash_n = dash_n as i32;
        }
        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }

        super::update_data_pair_from_json(&mut self.x, &mut self.y, rep);
    }

    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {

    }

    fn mapping_type(&self) -> String {
        "line".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String, String)> {
        vec![
            (String::from("x"), self.get_col_name("x")),
            (String::from("y"), self.get_col_name("y"))
        ]
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            _ => { }
        }
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        if cols.len() != 2 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            Ok(())
        }
    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymin = self.y.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymax = self.y.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}

