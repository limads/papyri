// use libxml::tree::node::Node;
use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
// use super::utils;
use super::*;
use std::cmp::*;
use std::default::Default;
use super::super::{MappingProperty, IntervalProperty};
use std::str::FromStr;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct IntervalMapping {
    color : RGBA,
    x : Vec<f64>,
    ymin : Vec<f64>,
    ymax : Vec<f64>,
    width : f64,
    dash_n : i32,
    lim_sz : f64,
    vertical : bool,
    col_names : [String; 3],
    source : String
}

impl Default for IntervalMapping {

    fn default() -> Self {
        Self {
            color : RGBA::black(),
            x : Vec::new(),
            ymin : Vec::new(),
            ymax : Vec::new(),
            width : 1.0,
            dash_n : 1,
            lim_sz : 1.0,
            col_names : [String::new(), String::new(), String::new()],
            source : String::new(),
            vertical : true
        }
    }

}

impl IntervalMapping {

    pub fn limit_size(mut self, sz : f64) -> Self {
        self.lim_sz = sz;
        self
    }

    pub fn vertical(mut self, vertical : bool) -> Self {
        self.vertical = vertical;
        self
    }

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

    pub fn map<D>(x : impl IntoIterator<Item=D>, ymin : impl IntoIterator<Item=D>, ymax : impl IntoIterator<Item=D>) -> Self
    where
        D : Borrow<f64>
    {
        let mut intv : IntervalMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.borrow() ).collect();
        let ymin : Vec<_> = ymin.into_iter().map(|d| *d.borrow() ).collect();
        let ymax : Vec<_> = ymax.into_iter().map(|d| *d.borrow() ).collect();
        intv.update_data(vec![x, ymin, ymax]);
        intv
    }

    /*pub fn new(node : &Node) -> Result<Self, String> {
        let color = gdk::RGBA{
            red:0.0,
            green:0.0,
            blue:0.0,
            alpha : 0.0
        };
        let width = 1.0;
        let dash_n = 1;
        let x = Vec::<f64>::new();
        let ymin = Vec::<f64>::new();
        let ymax = Vec::<f64>::new();
        let col_names = [
            String::from("None"),
            String::from("None"),
            String::from("None")
        ];
        let source = String::new();
        let lim_sz = 1.0;
        let vertical = true;
        let mut mapping = IntervalMapping{color, x, ymin, ymax, width, dash_n, col_names, source, lim_sz, vertical};
        mapping.update_layout(node)?;
        Ok(mapping)
    }*/

    fn build_dash(n : i32) -> Vec<f64> {
        let dash_sz = 10.0 / (n as f64);
        let mut dashes = Vec::<f64>::new();
        for _i in 1..n {
            dashes.push(dash_sz);
        }
        dashes
    }

}

impl Mapping for IntervalMapping {

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Interval(intv) => {
                match intv {
                    IntervalProperty::Color(col) => { self.color = col.parse().unwrap() },
                    IntervalProperty::Width(w) => { self.width = w },
                    IntervalProperty::Dash(d) => { self.dash_n = d },
                    IntervalProperty::Lower(l) => { self.ymin = l },
                    IntervalProperty::Upper(u) => { self.ymax = u },
                    IntervalProperty::Center(c) => { self.x = c },
                    IntervalProperty::Vertical(v) => { self.vertical = v },
                    IntervalProperty::Limit(l) => { self.lim_sz = l },
                }
                true
            },
            _ => false
        }
    }

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {
        //println!("{:?}", self);
        if self.x.len() < 1 || self.ymin.len() < 1 || self.ymax.len() < 1 {
            return;
        }
        ctx.save();
        ctx.set_source_rgb(
            self.color.red.into(),
            self.color.green.into(),
            self.color.blue.into()
        );
        ctx.set_line_width(self.width);
        let dashes = IntervalMapping::build_dash(self.dash_n);
        ctx.set_dash(&dashes[..], 0.0);

        let mut zip_xy = self.x.iter().zip(self.ymin.iter().zip(self.ymax.iter()));

        for (curr_x, (curr_ymin, curr_ymax)) in zip_xy {
            assert!(*curr_ymin <= *curr_ymax);
            if self.vertical {
                if mapper.check_bounds(*curr_x, *curr_ymin) && mapper.check_bounds(*curr_x, *curr_ymax) {
                    let from_low = mapper.map(*curr_x - self.lim_sz / 2., *curr_ymin);
                    let to_low = mapper.map(*curr_x + self.lim_sz / 2., *curr_ymin);
                    ctx.move_to(from_low.x, from_low.y);
                    ctx.line_to(to_low.x, to_low.y);
                    ctx.stroke();

                    let from_high = mapper.map(*curr_x - self.lim_sz / 2., *curr_ymax);
                    let to_high = mapper.map(*curr_x + self.lim_sz / 2., *curr_ymax);
                    ctx.move_to(from_high.x, from_high.y);
                    ctx.line_to(to_high.x, to_high.y);
                    ctx.stroke();

                    let from_bar = mapper.map(*curr_x, *curr_ymin);
                    let to_bar = mapper.map(*curr_x, *curr_ymax);
                    ctx.move_to(from_bar.x, from_bar.y);
                    ctx.line_to(to_bar.x, to_bar.y);
                    ctx.stroke();
                }
            } else {
                if mapper.check_bounds(*curr_ymin, *curr_x) && mapper.check_bounds(*curr_ymax, *curr_x) {
                    let from_low = mapper.map(*curr_ymin, *curr_x - self.lim_sz / 2.);
                    let to_low = mapper.map(*curr_ymin, *curr_x + self.lim_sz / 2.);
                    ctx.move_to(from_low.x, from_low.y);
                    ctx.line_to(to_low.x, to_low.y);
                    ctx.stroke();

                    let from_high = mapper.map(*curr_ymax, *curr_x - self.lim_sz / 2.);
                    let to_high = mapper.map(*curr_ymax, *curr_x + self.lim_sz / 2.);
                    ctx.move_to(from_high.x, from_high.y);
                    ctx.line_to(to_high.x, to_high.y);
                    ctx.stroke();

                    let from_bar = mapper.map(*curr_ymin, *curr_x);
                    let to_bar = mapper.map(*curr_ymax, *curr_x);
                    ctx.move_to(from_bar.x, from_bar.y);
                    ctx.line_to(to_bar.x, to_bar.y);
                    ctx.stroke();
                }
            }
        }
        ctx.restore();
    }

    fn update_data(&mut self, values : Vec<Vec<f64>>) {
        self.x = values[0].clone();
        self.ymin = values[1].clone();
        self.ymax = values[2].clone();
    }

    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {
        if let Some(width) = rep.width {
            self.width = width;
        }
        if let Some(dash_n) = rep.spacing {
            self.dash_n = dash_n;
        }
        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }
        if let Some(lim) = rep.limits.clone() {
            self.lim_sz = lim;
        }
        if let Some(vert) = rep.vertical.clone() {
            self.vertical = vert;
        }

        // println!("Mapping json rep: {:?}", rep);

        super::update_data_triplet_from_json(&mut self.x, &mut self.ymin, &mut self.ymax, rep);
        // TODO check properties of other mappings are None.
    }

    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {
        // println!("Mapping has no extra data");
    }

    /*fn update_layout(&mut self, node : &Node) -> Result<(), String> {
        let props = utils::children_as_hash(node, "property");
        self.color = props.get("color")
            .ok_or(format!("color property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse color property"))?;
        self.width = props.get("width")
            .ok_or(format!("width property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse width property"))?;
        self.dash_n = props.get("dash")
            .ok_or(format!("dash property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse dash property"))?;
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
    }*/

    fn properties(&self) -> HashMap<String, String> {
        let mut properties = MappingType::Line.default_hash();
        if let Some(e) = properties.get_mut("color") {
            *e = self.color.to_string();
        }
        if let Some(e) = properties.get_mut("width") {
            *e = self.width.to_string();
        }
        if let Some(e) = properties.get_mut("dash"){
            *e = self.dash_n.to_string();
        }
        if let Some(e) = properties.get_mut("x") {
            *e = self.col_names[0].clone();
        }
        if let Some(e) = properties.get_mut("ymin") {
            *e = self.col_names[1].clone();
        }
        if let Some(e) = properties.get_mut("ymax") {
            *e = self.col_names[1].clone();
        }
        if let Some(e) = properties.get_mut("source") {
            *e = self.source.clone();
        }
        properties
    }

    fn mapping_type(&self) -> String {
        "interval".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "ymin" => self.col_names[1].clone(),
            "ymax" => self.col_names[2].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String, String)> {
        /*vec![
            (String::from("x"), self.get_col_name("x")),
            (String::from("ymin"), self.get_col_name("ymin"))
            (String::from("ymax"), self.get_col_name("ymax"))
        ]*/
        unimplemented!()
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        /*let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols*/
        unimplemented!()
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        /*match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            _ => { }
        }*/
        unimplemented!()
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        /*if cols.len() != 2 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            Ok(())
        }*/
        unimplemented!()
    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        if self.vertical {
            let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )? - self.lim_sz / 2.;
            let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))? + self.lim_sz / 2.;
            let ymin = self.ymin.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            let ymax = self.ymax.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            Some(((xmin, xmax), (*ymin, *ymax)))
        } else {
            let ymin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )? - self.lim_sz / 2.;
            let ymax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))? + self.lim_sz / 2.;
            let xmin = self.ymin.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            let xmax = self.ymax.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
            Some(((*xmin, *xmax), (ymin, ymax)))
        }
    }

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}

