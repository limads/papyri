// use libxml::tree::node::Node;
use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
// use std::f64::consts::PI;
// use super::utils;
// use super::super::context_mapper::Coord2D;
// use cairo::ScaledFont;
// use super::super::text::{FontData, draw_label};
use super::*;
//use crate::mappings::other::Mapping;
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
            color : RGBA::black(),
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

    /*pub fn new(node : &Node) -> Result<Self, String> {
        let x = Vec::<f64>::new();
        let ymin = Vec::<f64>::new();
        let ymax = Vec::<f64>::new();
        let color = gdk::RGBA{
            red:0.0,
            green:0.0,
            blue:0.0,
            alpha : 1.0
        };
        let col_names = [
            String::from("None"),
            String::from("None"),
            String::from("None")
        ];
        let source = String::new();
        let mut mapping = AreaMapping{ x, ymin, ymax, color, col_names, source };
        mapping.update_layout(node)?;
        Ok(mapping)
    }*/

    pub fn draw_bound<'a>(
        pts : impl Iterator<Item = ((&'a f64, &'a f64), (&'a f64, &'a f64))>,
        mapper : &ContextMapper,
        ctx : &Context
    ) {
        for ((x0, y0), (x1, y1)) in pts {
            let bounds_ok = mapper.check_bounds(*x0, *y0) &&
                mapper.check_bounds(*x1, *y1);
            if bounds_ok {
                let from = mapper.map(*x0, *y0);
                let to   = mapper.map(*x1, *y1);
                // println!("{:?}", (from, to));
                // ctx.move_to(from.x, from.y);
                ctx.line_to(to.x, to.y);
                //ctx.stroke();
            } else {
                //println!("Out of bounds mapping");
            }
        }
    }

}

/*pub fn get_first_point(
    zip_xy : &mut dyn Iterator<Item = (f64, f64)>
) -> Option<(f64, f64)> {

}*/

impl Mapping for AreaMapping {

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(line) => {

                true
            },
            _ => false
        }
    }

    // Mapping-specific impl.
    // fn new(node : Node&) -> Self;

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update_from_json(&mut self, mut rep : crate::model::Mapping) {
        // if let Some(ymin) = rep.ymin {
        // }
        // if let Some(ymax) = rep.ymax  {
        // }
        // TODO check properties of other mappings are None.
        unimplemented!()
    }

    // Mapping-specific impl.
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {
        ctx.save();
        ctx.set_source_rgba(
            self.color.red.into(),
            self.color.green.into(),
            self.color.blue.into(),
            self.color.alpha.into()
        );
        ctx.set_fill_rule(cairo::FillRule::Winding);
        if self.x.len() == 0 {
            ctx.restore();
            return;
        }
        let pt0 = mapper.map(self.x[0], self.ymin[0]);
        ctx.move_to(pt0.x, pt0.y);
        //println!("Received for drawing {:?} {:?}", self.x, self.y);
        let zip_xy0 = self.x.iter().zip(self.ymin.iter());
        let zip_xy1 = self.x.iter().skip(1).zip(self.ymin.iter().skip(1));
        AreaMapping::draw_bound(zip_xy0.zip(zip_xy1), mapper, ctx);
        match (self.x.last(), self.ymin.last(), self.ymax.last()) {
            (Some(x), Some(_ymin), Some(ymax)) => {
                // let from = mapper.map(*x, *ymin);
                let to = mapper.map(*x, *ymax);
                //ctx.move_to(from.x, from.y);
                ctx.line_to(to.x, to.y);
            },
            _ => { ctx.restore(); return; }
        }
        let zip_xy0_rev = self.x.iter().rev().zip(self.ymax.iter().rev());
        let zip_xy1_rev = self.x.iter().rev().skip(1).zip(self.ymax.iter().rev().skip(1));
        AreaMapping::draw_bound(zip_xy0_rev.zip(zip_xy1_rev), mapper, ctx);
        let _pt0 = mapper.map(self.x[0], self.ymax[0]);
        let pt = mapper.map(self.x[0], self.ymin[0]);
        ctx.line_to(pt.x, pt.y);
        ctx.close_path();
        //ctx.fill_preserve();
        ctx.fill();
        ctx.restore();
    }

    // Mapping-specific impl.
    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.ymin = values[1].clone();
        self.ymax = values[2].clone();
    }

    /*fn update_layout(&mut self, node : &Node) -> Result<(), String> {
        let props = utils::children_as_hash(node, "property");
        self.color = props.get("color")
            .ok_or(format!("Color property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse opacity property"))?;
        self.color.alpha = props.get("opacity")
            .ok_or(format!("Color opacity not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse opacity property"))?;
        self.col_names[0] = props.get("x")
            .ok_or(format!("x property not found"))?
            .clone();
        self.col_names[1] = props.get("y")
            .ok_or(format!("y property not found"))?
            .clone();
        self.col_names[2] = props.get("ymax")
            .ok_or(format!("ymax opacity not found"))?
            .clone();
        self.source = props.get("source")
            .ok_or(format!("Source property not found"))?
            .clone();
        Ok(())
    }*/

    fn properties(&self) -> HashMap<String, String> {
        let mut properties = MappingType::Area.default_hash();
        if let Some(e) = properties.get_mut("color") {
            *e = self.color.to_string();
        }
        if let Some(e) = properties.get_mut("opacity") {
            *e = self.color.alpha.to_string();
        }
        if let Some(e) = properties.get_mut("x") {
            *e = self.col_names[0].clone();
        }
        if let Some(e) = properties.get_mut("y") {
            *e = self.col_names[1].clone();
        }
        if let Some(e) = properties.get_mut("ymax") {
            *e = self.col_names[2].clone();
        }
        if let Some(e) = properties.get_mut("source") {
            *e = self.source.clone();
        }
        properties
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
        // println!("Mapping has no extra data");
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
