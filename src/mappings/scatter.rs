use libxml::tree::node::Node;
use gdk::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
use std::f64::consts::PI;
use super::utils;
use super::*;
use std::cmp::*;
use std::str::FromStr;
use super::super::{MappingProperty, ScatterProperty};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct ScatterMapping {
    color : RGBA,
    x : Vec<f64>,
    y : Vec<f64>,
    radius : f64,
    col_names : [String; 2],
    source : String
}

impl Default for ScatterMapping {

    fn default() -> Self {
        Self {
            color : RGBA::black(),
            x : Vec::new(),
            y : Vec::new(),
            radius : 1.0,
            col_names : [String::new(), String::new()],
            source : String::new()
        }
    }

}

impl ScatterMapping {

    pub fn color(mut self, color : String) -> Self {
        self.color = color.parse().unwrap();
        self
    }

    pub fn radius(mut self, radius : f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>) -> Self
    where
        D : Borrow<f64>
    {
        let mut scatter : ScatterMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.borrow() ).collect();
        let y : Vec<_> = y.into_iter().map(|d| *d.borrow() ).collect();
        scatter.update_data(vec![x, y]);
        scatter
    }

    pub fn new(node : &Node) -> Result<Self,String> {
        let color = RGBA::black();
        let radius = 1.0;
        let x = Vec::<f64>::new();
        let y = Vec::<f64>::new();
        let col_names = [
            String::from("None"),
            String::from("None")
        ];
        let source = String::new();
        let mut mapping = ScatterMapping{color, x, y, radius, col_names, source};
        mapping.update_layout(node)?;
        Ok(mapping)
    }
}

impl Mapping for ScatterMapping {

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Scatter(scatter) => {
                match scatter {
                    ScatterProperty::Color(col) => { self.color = col.parse().unwrap() },
                    ScatterProperty::Radius(r) => { self.radius = r },
                    ScatterProperty::X(x) => { self.x = x },
                    ScatterProperty::Y(y) => { self.y = y }
                }
                true
            },
            _ => false
        }
    }

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {
        // TODO check properties of other mappings are None.
        if let Some(color) = rep.color.clone() {
            self.color = RGBA::from_str(&color).unwrap();
        }
        if let Some(radius) = rep.radius {
            self.radius = radius;
        }

        // println!("Scatter mapping json rep: {:?}", rep);

        super::update_data_pair_from_json(&mut self.x, &mut self.y, rep);
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {
        ctx.save();
        ctx.set_source_rgba(
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha
        );
        for (x, y) in self.x.iter().zip(self.y.iter()) {
            if mapper.check_bounds(*x, *y) {
                let pos = mapper.map(*x, *y);
                ctx.arc(pos.x, pos.y, self.radius, 0.0, 2.0*PI);
                ctx.fill();
                ctx.stroke();
            } else {
                println!("Out of bounds mapping");
            }
        }
        ctx.restore();
    }

    //fn new(&self, HashMap<String, String> properties);
    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.y = values[1].clone();
    }

    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {
        // println!("Mapping has no extra data");
    }

    fn update_layout(&mut self, node : &Node) -> Result<(), String> {
        let props = utils::children_as_hash(node, "property");
        self.color = props.get("color")
            .ok_or(format!("color property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse color property"))?;
        self.radius = props.get("radius")
            .ok_or(format!("radius property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse radius property"))?;
        self.col_names[0] = props.get("x")
            .ok_or(format!("x property not found"))?
            .clone();
        self.col_names[1] = props.get("y")
            .ok_or(format!("y property not found"))?
            .clone();
        self.source = props.get("source")
            .ok_or(format!("Source property not found"))?
            .clone();
        Ok(())
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut properties = MappingType::Scatter.default_hash();
        if let Some(e) = properties.get_mut("color") {
            *e = self.color.to_string();
        }
        if let Some(e) = properties.get_mut("radius") {
            *e = self.radius.to_string();
        }
        if let Some(e) = properties.get_mut("x") {
            *e = self.col_names[0].clone();
        }
        if let Some(e) = properties.get_mut("y") {
            *e = self.col_names[1].clone();
        }
        if let Some(e) = properties.get_mut("source") {
            *e = self.source.clone();
        }
        properties
    }

    fn mapping_type(&self) -> String {
        "scatter".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            _ => String::new()
        }
    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let ymin = self.y.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let ymax = self.y.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }

    fn get_ordered_col_names(&self) -> Vec<(String,String)> {
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

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }

}

