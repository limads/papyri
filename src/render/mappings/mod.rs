/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

// use libxml::tree::node::Node;
use cairo::Context;
use super::context_mapper::ContextMapper;
use std::collections::HashMap;
// use super::utils;
use super::text::{FontData, draw_label};
use super::MappingProperty;
use std::cmp::Ordering;
use std::default::Default;
use std::mem;
use std::error::Error;
use std::fmt::Debug;
use crate::render::PlotError;

pub mod area;

pub mod bar;

pub mod line;

pub mod scatter;

// pub mod surface;

pub mod text;

pub mod interval;

fn update_single_data_from_json(x : &mut Vec<f64>, mut rep : crate::model::Mapping) {
    if let Some(new_x) = mem::take(&mut rep.map.x) {
        *x = new_x;
    }
}

fn update_data_pair_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, mut rep : crate::model::Mapping) {
    if let Some(new_x) = mem::take(&mut rep.map.x) {
        *x = new_x;
    }
    if let Some(new_y) = mem::take(&mut rep.map.y) {
        *y = new_y;
    }
}

fn update_data_triplet_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<f64>, mut rep : crate::model::Mapping) {
    if let Some(new_x) = mem::take(&mut rep.map.x) {
        *x = new_x;
    } else {
        println!("Missing x");
    }
    if let Some(new_y) = mem::take(&mut rep.map.y) {
        *y = new_y;
    } else {
        println!("Missing y");
    }
    if let Some(new_z) = mem::take(&mut rep.map.z) {
        *z = new_z;
    } else {
        println!("Missing z");
    }
}

fn update_textual_data_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<String>, mut rep : crate::model::Mapping) {
    if let Some(new_x) = mem::take(&mut rep.map.x) {
        *x = new_x;
    }
    if let Some(new_y) = mem::take(&mut rep.map.y) {
        *y = new_y;
    }
    if let Some(new_text) = mem::take(&mut rep.map.text) {
        *z = new_text;
    }
}

pub fn new_from_json(rep : crate::model::Mapping) -> Result<Box<dyn Mapping>, Box<dyn Error>> {
    // Must be line|scatter|area|bar|surface|text
    let mut mapping : Box<dyn Mapping> = match &rep.kind[..] {
        "line" => {
            let line : line::LineMapping = Default::default();
            Box::new(line)
        },
        "scatter" => {
            let scatter : scatter::ScatterMapping = Default::default();
            Box::new(scatter)
        },
        "area" => {
            let area : area::AreaMapping = Default::default();
            Box::new(area)
        },
        "bar" => {
            let bar : bar::BarMapping = Default::default();
            Box::new(bar)
        },
        // "surface" => {
        //    let surface : surface::SurfaceMapping = Default::default();
        //    Box::new(surface)
        // },
        "text" => {
            let text : text::TextMapping = Default::default();
            Box::new(text)
        },
        "interval" => {
            let intv : interval::IntervalMapping = Default::default();
            Box::new(intv)
        },
        _ => {
            return Err(Box::new(PlotError::InvalidData("Invalid mapping type")));
        }
    };
    rep.validate()?;
    mapping.update_from_json(rep);
    Ok(mapping)
}

/// Default trait for updating mappings.
/// Mappings are always instantiated from
/// the concrete instances new(.) call, which
/// receive a XML definition. To aid in creating
/// this definition, the MappingType::default_hash function
/// can be used.
pub trait Mapping
where
    Self : Debug
{

    // Mapping-specific impl.
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> ;// { }

    // We cannot simply require Self : Clone because this
    // assumes Self : Sized, which makes it impossible to use
    // trait objects. clone_boxed just clones the underlying mapping
    // and Box it. We implement Clone for Box<dyn Mapping> afterwards,
    // which just calls this method.
    fn clone_boxed(&self) -> Box<dyn Mapping>;

    fn update(&mut self, prop : MappingProperty) -> bool;

    // Mapping-specific impl.
    fn update_data(&mut self, values : Vec<Vec<f64>>); //{ }

    fn update_extra_data(&mut self, values : Vec<Vec<String>>);

    // fn update_layout(&mut self, node : &Node) -> Result<(), String>;

    // fn properties(&self) -> HashMap<String, String>;

    fn mapping_type(&self) -> String;

    fn get_col_name(&self, col : &str) -> String;

    fn get_ordered_col_names(&self) -> Vec<(String, String)>;

    fn get_hash_col_names(&self) -> HashMap<String, String>;

    fn set_col_name(&mut self, col : &str, name : &str);

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str>;

    fn set_source(&mut self, source : String);

    fn get_source(&self) -> String;

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))>;

    fn update_from_json(&mut self, rep : crate::model::Mapping);

}

impl Clone for Box<dyn Mapping> {

    fn clone(&self) -> Self {
        self.clone_boxed()
    }

}

//fn compare(a : &f64, b : &f64) -> Ordering {
//    a.partial_cmp(b).unwrap_or(Ordering::Equal)
//}


