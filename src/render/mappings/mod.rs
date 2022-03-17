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
use base64;

pub mod area;

pub mod bar;

pub mod line;

pub mod scatter;

pub mod surface;

pub mod text;

pub mod interval;

pub enum MappingType {
    Line,
    Scatter,
    Bar,
    Area,
    Surface,
    Text,
    Interval
}

impl MappingType {

    pub fn from_str(name : &str) -> Option<Self> {
        match name {
            "line" => Some(MappingType::Line),
            "scatter" => Some(MappingType::Scatter),
            "bar" => Some(MappingType::Bar),
            "area" => Some(MappingType::Area),
            "surface" => Some(MappingType::Surface),
            "text" => Some(MappingType::Text),
            "interval" => Some(MappingType::Interval),
            _ => None
        }
    }

    /// Returns a default property map for this mapping type. This is the major
    /// reference for the validity of any given plot property. This function
    /// deals with non-data properties.
    pub fn default_hash(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        hash.insert(String::from("color"), String::from("#000000"));
        hash.insert(String::from("x"), String::from("None"));
        hash.insert(String::from("y"), String::from("None"));
        hash.insert(String::from("source"), String::from("None"));
        match self {
            MappingType::Line => {
                hash.insert(String::from("width"), String::from("1"));
                hash.insert(String::from("dash"), String::from("1"));
            },
            MappingType::Scatter => {
                hash.insert(String::from("radius"), String::from("1"));
            },
            MappingType::Bar => {
                hash.insert(String::from("center_anchor"), String::from("false"));
                hash.insert(String::from("horizontal"), String::from("false"));
                hash.insert(String::from("width"), String::from("None"));
                hash.insert(String::from("height"), String::from("None"));
                hash.insert(String::from("bar_width"), String::from("100"));
                hash.insert(String::from("origin_x"), String::from("0"));
                hash.insert(String::from("origin_y"), String::from("0"));
                hash.insert(String::from("bar_spacing"), String::from("1"));
            },
            MappingType::Area => {
                hash.insert(String::from("ymax"), String::from("None"));
                hash.insert(String::from("opacity"), String::from("1.0"));
            },
            MappingType::Surface => {
                hash.insert(String::from("z"), String::from("None"));
                hash.insert(String::from("final_color"), String::from("#ffffff"));
                hash.insert(String::from("z_min"), String::from("0.0"));
                hash.insert(String::from("z_max"), String::from("1.0"));
                hash.insert(String::from("opacity"), String::from("1.0"));
            },
            MappingType::Text => {
                hash.insert(String::from("font"), String::from("Monospace Regular 12"));
                hash.insert(String::from("text"), String::from("None"));
            },
            MappingType::Interval => {
                unimplemented!()
            }
        }
        hash
    }
}

fn update_data_pair_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, mut rep : crate::model::Mapping) {
    if let Some(ref mut map) = rep.map {
        if let Some(new_x) = mem::take(&mut map.x) {
            *x = new_x;
        }
        if let Some(new_y) = mem::take(&mut map.y) {
            *y = new_y;
        }
    }
}

fn update_data_triplet_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<f64>, mut rep : crate::model::Mapping) {
    if let Some(ref mut map) = rep.map {
        if let Some(new_x) = mem::take(&mut map.x) {
            *x = new_x;
        } else {
            println!("Missing x");
        }
        if let Some(new_y) = mem::take(&mut map.y) {
            *y = new_y;
        } else {
            println!("Missing y");
        }
        if let Some(new_z) = mem::take(&mut map.z) {
            *z = new_z;
        } else {
            println!("Missing z");
        }
    }
}

fn update_textual_data_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<String>, mut rep : crate::model::Mapping) {
    if let Some(ref mut map) = rep.map {
        if let Some(new_x) = mem::take(&mut map.x) {
            *x = new_x;
        }
        if let Some(new_y) = mem::take(&mut map.y) {
            *y = new_y;
        }
        if let Some(new_text) = mem::take(&mut map.text) {
            *z = new_text;
        }
    }
}

pub fn new_from_json(mut rep : crate::model::Mapping) -> Result<Box<dyn Mapping>, Box<dyn Error>> {
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
        "surface" => {
            let surface : surface::SurfaceMapping = Default::default();
            Box::new(surface)
        },
        "text" => {
            let text : text::TextMapping = Default::default();
            Box::new(text)
        },
        "interval" => {
            let intv : interval::IntervalMapping = Default::default();
            Box::new(intv)
        },
        _ => panic!("Unrecognized mapping")
    };
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
    fn draw(&self, mapper : &ContextMapper, ctx : &Context);// { }

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

    fn properties(&self) -> HashMap<String, String>;

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


