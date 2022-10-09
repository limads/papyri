/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
use super::*;
use std::cmp::*;
use super::super::MappingProperty;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct AreaMapping {
    x : Vec<f64>,
    ymin : Vec<f64>,
    ymax : Vec<f64>,
    color : RGBA,
    col_names : [String; 3],
    source : String
}

impl Default for AreaMapping {

    fn default() -> Self {
        Self {
            color : RGBA::BLACK,
            x : Vec::new(),
            ymin : Vec::new(),
            ymax : Vec::new(),
            col_names : [String::new(), String::new(), String::new()],
            source : String::new()
        }
    }

}

impl AreaMapping {

    pub fn color(mut self, color : String) -> Self {
        self.color = color.parse().unwrap();
        self
    }

    pub fn map<D>(x : impl IntoIterator<Item=D>, ymin : impl IntoIterator<Item=D>, ymax : impl IntoIterator<Item=D>) -> Self
    where
        D : Borrow<f64>
    {
        let mut area : AreaMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.borrow() ).collect();
        let ymin : Vec<_> = ymin.into_iter().map(|d| *d.borrow() ).collect();
        let ymax : Vec<_> = ymax.into_iter().map(|d| *d.borrow() ).collect();
        area.update_data(vec![x, ymin, ymax]);
        area
    }

    pub fn draw_bound<'a>(
        pts : impl Iterator<Item = ((&'a f64, &'a f64), (&'a f64, &'a f64))>,
        mapper : &ContextMapper,
        ctx : &Context
    ) {
        for ((x0, y0), (x1, y1)) in pts {
            let bounds_ok = mapper.check_bounds(*x0, *y0) &&
                mapper.check_bounds(*x1, *y1);
            if bounds_ok {
                let _from = mapper.map(*x0, *y0);
                let to   = mapper.map(*x1, *y1);
                ctx.line_to(to.x, to.y);
            } else {
                // eprintln!("Out of bounds mapping");
            }
        }
    }

}

impl Mapping for AreaMapping {

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(_line) => {

                true
            },
            _ => false
        }
    }

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update_from_json(&mut self, rep : crate::model::Mapping) {
        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }
        super::update_data_triplet_from_json(&mut self.x, &mut self.ymin, &mut self.ymax, rep);
    }

    // Mapping-specific impl.
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
        ctx.set_source_rgba(
            self.color.red().into(),
            self.color.green().into(),
            self.color.blue().into(),
            self.color.alpha().into()
        );
        ctx.set_fill_rule(cairo::FillRule::Winding);
        if self.x.len() == 0 {
            ctx.restore()?;
            return Ok(());
        }
        let pt0 = mapper.map(self.x[0], self.ymin[0]);
        ctx.move_to(pt0.x, pt0.y);
        let zip_xy0 = self.x.iter().zip(self.ymin.iter());
        let zip_xy1 = self.x.iter().skip(1).zip(self.ymin.iter().skip(1));
        AreaMapping::draw_bound(zip_xy0.zip(zip_xy1), mapper, ctx);
        match (self.x.last(), self.ymin.last(), self.ymax.last()) {
            (Some(x), Some(_ymin), Some(ymax)) => {
                let to = mapper.map(*x, *ymax);
                ctx.line_to(to.x, to.y);
            },
            _ => {
                ctx.restore()?;
                return Ok(());
            }
        }
        let zip_xy0_rev = self.x.iter().rev().zip(self.ymax.iter().rev());
        let zip_xy1_rev = self.x.iter().rev().skip(1).zip(self.ymax.iter().rev().skip(1));
        AreaMapping::draw_bound(zip_xy0_rev.zip(zip_xy1_rev), mapper, ctx);
        let _pt0 = mapper.map(self.x[0], self.ymax[0]);
        let pt = mapper.map(self.x[0], self.ymin[0]);
        ctx.line_to(pt.x, pt.y);
        ctx.close_path();
        ctx.fill()?;
        ctx.restore()?;
        Ok(())
    }

    // Mapping-specific impl.
    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.ymin = values[1].clone();
        self.ymax = values[2].clone();
    }

    fn mapping_type(&self) -> String {
        "area".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            "ymax" => self.col_names[2].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String, String)> {
        vec![
            (String::from("x"),self.get_col_name("x")),
            (String::from("y"), self.get_col_name("y")),
            (self.get_col_name("ymax"), String::from("ymax"))
        ]
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols.insert("ymax".into(), self.col_names[2].clone());
        cols
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            "ymax" => { self.col_names[2] = name.into(); },
            _ => { }
        }
    }

    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {
    
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        if cols.len() != 3 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            self.set_col_name("ymax", &cols[2]);
            Ok(())
        }
    }

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymin = self.ymin.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymax = self.ymax.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }
}

