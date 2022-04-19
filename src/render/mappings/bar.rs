// use libxml::tree::node::Node;
use gdk4::RGBA;
use cairo::Context;
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
// use super::utils;
use super::*;
use std::mem;
use super::super::MappingProperty;
use std::borrow::Borrow;

/// Represents ordered, regularly-spaced bars. The only map required by the bars
/// is their height (or width if they are horizontally spaced), since the bar position
/// is fully determined by its order in the data array, the informed offset and bar spacing (at data scale).
#[derive(Debug, Clone)]
pub struct BarMapping {
    color : RGBA,
    center_anchor : bool,
    x : Vec<f64>,
    y : Vec<f64>,
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
            color : RGBA::black(),
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

    /*pub fn new(node : &Node) -> Result<Self, String> {
        let color = gdk::RGBA{
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha : 0.0
        };
        let x = Vec::<f64>::new();
        let y = Vec::<f64>::new();
        let w = Vec::<f64>::new();
        let h = Vec::<f64>::new();
        let center_anchor = false;
        let col_names = [
            String::from("None"),
            String::from("None"),
            String::from("None"),
            String::from("None")
        ];
        let bar_width = 100.0;
        let origin = (0.0, 0.0);
        let bar_spacing = 1.0;
        let horizontal = false;
        let source = String::new();
        let mut mapping = BarMapping{
            color, center_anchor, x, y, w, h,
            col_names, bar_width, origin, bar_spacing, horizontal,
            source
        };
        mapping.update_layout(node)?;
        Ok(mapping)
    }*/

    fn adjust_bar(&mut self) {
        if self.horizontal {
            let n = self.w.len();
            self.y = (0..n).map(|i| self.origin.1 + self.bar_spacing * i as f64 ).collect();
            self.x = (0..n).map(|_| self.origin.0 ).collect();
            if self.center_anchor {
                let spacing = self.bar_spacing;
                self.y.iter_mut().for_each(|y| *y -= spacing / 2. );
            }
        } else {
            let n = self.h.len();
            self.y = (0..n).map(|_| self.origin.1 ).collect();
            self.x = (0..n).map(|i| self.origin.0  + self.bar_spacing * i as f64 ).collect();
            if self.center_anchor {
                let spacing = self.bar_spacing;
                self.x.iter_mut().for_each(|x| *x -= spacing / 2. );
            }
        }
        if self.horizontal {
            self.h = (0..self.w.len()).map(|_| self.bar_spacing * self.bar_width / 100. ).collect();
        } else {
            self.w = (0..self.h.len()).map(|_| self.bar_spacing * self.bar_width / 100. ).collect();
        }
    }

}

impl Mapping for BarMapping {

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(line) => {
                unimplemented!()
            },
            _ => false
        }
    }

    fn update_from_json(&mut self, mut rep : crate::model::Mapping) {
        // TODO check properties of other mappings are None.

        if let Some(w) = rep.bar_width {
            self.bar_width = w;
        }

        if let Some(h) = rep.horizontal {
            self.horizontal = h;
        }

        if let Some(s) = rep.bar_spacing {
            self.bar_spacing = s;
        }

        if let Some(origin) = rep.origin {
            if self.horizontal {
                self.origin = (0., origin);
            } else {
                self.origin = (origin, 0.);
            }
        }

        if let Some(c) = rep.center_anchor {
            self.center_anchor = c;
        }

        if let Some(color) = rep.color.clone() {
            self.color = color.parse().unwrap();
        }

        let mut new_data = Vec::new();
        super::update_single_data_from_json(&mut new_data, rep);
        self.update_data(vec![new_data]);
    }

    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {
        ctx.save();
        ctx.set_source_rgb(self.color.red.into(), self.color.green.into(), self.color.blue.into());
        //println!("Received for drawing {:?} {:?} {:?} {:?}", self.x, self.y, self.w, self.h);
        let r_iter = self.x.iter().zip(self.y.iter()
            .zip(self.w.iter()
            .zip(self.h.iter()))
        );
        for (x, (y, (w, h))) in r_iter {
            //let x_off = match self.center_anchor {
            //    false => *x,
            //    true => *x - *w / 2.0
            //};
            let tl_ok = mapper.check_bounds(*x, y + h);
            let tr_ok = mapper.check_bounds(x + w, y + h);
            let bl_ok = mapper.check_bounds(*x, *y);
            let br_ok = mapper.check_bounds(x + *w, *y);
            if  tl_ok && tr_ok && bl_ok && br_ok {
                let bottom_left = mapper.map(*x, *y);
                let bottom_right = mapper.map(x + *w, *y);
                let top_left = mapper.map(*x, *y + *h);
                //let top_right = mapper.map(x_off + *w, *y + *h);
                let coord_w = bottom_left.distance(bottom_right);
                let coord_h = bottom_left.distance(top_left);
                ctx.rectangle(top_left.x, top_left.y, coord_w, coord_h);
                ctx.fill();
                ctx.stroke();
            } else {
                // println!("Out of bounds mapping");
            }
        }
        ctx.restore();
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
        // println!("Mapping has no extra data");
    }

    /*fn update_layout(&mut self, node : &Node) -> Result<(), String> {
        let props = utils::children_as_hash(node, "property");
        self.color = props.get("color")
            .ok_or(format!("Color property not found"))?
            .parse().unwrap();
        self.center_anchor = props.get("center_anchor")
            .ok_or(format!("Center anchor property not found"))?
            .parse().unwrap();
        self.col_names[0] = props.get("x")
            .ok_or(format!("x property not found"))?
            .clone();
        self.col_names[1] = props.get("y")
            .ok_or(format!("y property not found"))?
            .clone();
        self.col_names[2] = props.get("width")
            .ok_or(format!("width property not found"))?
            .clone();
        self.col_names[3] = props.get("height")
            .ok_or(format!("height property not found"))?
            .clone();
        self.origin.0 = props.get("origin_x")
            .ok_or(format!("origin_x property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse origin_x property"))?;
        self.origin.1 = props.get("origin_y")
            .ok_or(format!("origin_y property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse origin_y property"))?;
        self.bar_width = props.get("bar_width")
            .ok_or(format!("bar_width property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse bar_width property"))?;
        self.bar_spacing = props.get("bar_spacing")
            .ok_or(format!("bar_spacing property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse bar_spacing property"))?;
        let new_horiz = props.get("horizontal")
            .ok_or(format!("horizontal property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse horizontal property"))?;
        if self.horizontal != new_horiz {
            mem::swap(&mut self.w, &mut self.h);
            self.horizontal = new_horiz;
        }
        self.source = props.get("source")
            .ok_or(format!("Source property not found"))?
            .clone();
        self.adjust_bar();
        /*println!("x: {:?}", self.x);
        println!("y: {:?}", self.y);
        println!("w: {:?}", self.w);
        println!("h: {:?}", self.h);*/
        Ok(())
    }*/

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymin = self.y.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymax = self.y.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut properties = MappingType::Bar.default_hash();
        if let Some(e) = properties.get_mut("color") {
            *e = self.color.to_string();
        }
        if let Some(e) = properties.get_mut("center_anchor") {
            *e = self.center_anchor.to_string(); // verify if returns "true" "false" here
        }
        if let Some(e) = properties.get_mut("x") {
            *e = self.col_names[0].clone();
        }
        if let Some(e) = properties.get_mut("y") {
            *e = self.col_names[1].clone();
        }
        if let Some(e) = properties.get_mut("width") {
            *e = self.col_names[2].clone();
        }
        if let Some(e) = properties.get_mut("height") {
            *e = self.col_names[3].clone();
        }
        if let Some(e) = properties.get_mut("origin_x") {
            *e = self.origin.0.to_string();
        }
        if let Some(e) = properties.get_mut("origin_y") {
            *e = self.origin.1.to_string();
        }
        if let Some(e) = properties.get_mut("bar_width") {
            *e = self.bar_width.to_string();
        }
        if let Some(e) = properties.get_mut("bar_spacing") {
            *e = self.bar_spacing.to_string();
        }
        if let Some(e) = properties.get_mut("horizontal") {
            *e = self.horizontal.to_string();
        }
        if let Some(e) = properties.get_mut("source") {
            *e = self.source.clone();
        }
        properties
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
            /*self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            self.set_col_name("width", &cols[2]);
            self.set_col_name("height", &cols[3]);*/
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

