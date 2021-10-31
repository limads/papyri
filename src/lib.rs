use cairo::Context;
use libxml::tree::document::{Document, SaveOptions};
use libxml::parser::Parser;
use libxml::tree::node::Node;
use std::default::Default;
use std::collections::HashMap;
use std::error::Error;
use std::rc::*;
use std::cell::*;
use std::result::Result;
use std::io::{ErrorKind, Write};
use mappings::area::*;
use mappings::*;
use context_mapper::{ContextMapper, Coord2D};
pub use scale::*;
use plot_design::*;
use std::fmt::Display;
mod text;
use std::any::Any;
use std::error;
use std::{fmt, fs::File};
use cairo::{SvgSurface, PsSurface, ImageSurface, Format};
use std::path::Path;
use std::cmp::Ordering;
use std::mem;
use std::str::FromStr;
use either::Either;
use text::FontData;

pub mod mappings;

// pub mod plot_view;

pub mod context_mapper;

pub mod plot_design;

pub mod scale;

pub use mappings::bar::*;

pub use mappings::scatter::*;

pub use mappings::line::*;

pub use mappings::surface::*;

pub use mappings::text::*;

//use sync::*;
/*impl Mapping for BarMapping {
}*/

/*#[derive(Debug)]
pub enum PlotError {
    PropertyNotFound,
    ViolateBounds
}

impl Display for PlotError {

}

impl Error for PlotError {

}*/

/// Contains structures used for deserialization of a JSON plot definition
pub mod json {

    use serde::{Serialize, Deserialize};
    use std::default::Default;
    use either::Either;
    use super::Adjustment;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Design {
        pub bg_color : String,
        pub grid_color : String,
        pub grid_width : i32,
        pub font : String
    }

    impl Default for Design {
        fn default() -> Self {
            Self {
                bg_color : String::from("#ffffff"),
                grid_color : String::from("#d3d7cf"),
                grid_width : 1,
                font : String::from("Monospace Regular 12")
            }
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Layout {
        pub width : i32,
        pub height : i32,
        pub horizontal_ratio : f64,
        pub vertical_ratio : f64
    }

    impl Default for Layout {
        fn default() -> Self {
            Self {
                width : 800,
                height : 600,
                horizontal_ratio : 0.5,
                vertical_ratio : 0.5
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Scale {
        pub label : String,
        pub precision : i32,
        pub from : f64,
        pub to : f64,
        pub n_intervals : i32,
        pub log : bool,
        pub invert : bool,
        pub offset : i32,
        pub adjust : Option<String>
    }

    impl Default for Scale {
        fn default() -> Self {
            Self {
                label : String::new(),
                precision : 2,
                from : 0.0,
                to : 1.0,
                n_intervals : 5,
                log : false,
                invert : false,
                offset : 0,
                adjust : None
            }
        }
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Map {

        pub x : Option<Vec<f64>>,
        pub y : Option<Vec<f64>>,

        //surface and area-specific
        pub z : Option<Vec<f64>>,

        pub text : Option<Vec<String>>
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Mapping {

        // Must be line|scatter|area|bar|surface|text
        pub kind : String,

        pub color : Option<String>,

        pub map : Option<Map>,

        // area-specific
        pub ymin : Option<f64>,

        pub ymax : Option<f64>,

        // text-specific
        pub text : Option<Vec<String>>,
        pub font : Option<String>,

        // Scatter-specific
        pub radius : Option<f64>,

        // Line-specific
        pub width : Option<f64>,
        pub spacing : Option<i32>,

    }

    // Plot carries design only if not within a larger panel.
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Plot {
        pub mappings : Vec<Mapping>,
        pub x : Scale,
        pub y : Scale,
        pub design : Option<Design>,
        pub layout : Option<Layout>
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Panel {
        // TODO add this field and derive deserialize here manually.
        // pub elements : Vec<Either<Box<Panel>, Plot>>,
        pub elements : Vec<Panel>,
        pub design : Option<Design>,
        pub layout : Option<Layout>
    }

}

/// Move old libxml logic to here.
mod xml {

}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GroupSplit {
    Unique,
    Vertical,
    Horizontal,
    Four,
    ThreeLeft,
    ThreeTop,
    ThreeRight,
    ThreeBottom
}

impl FromStr for GroupSplit {

    type Err = ();

    fn from_str(s : &str) -> Result<Self, ()> {
        match s {
            "Unique" => Ok(Self::Unique),
            "Four" => Ok(Self::Four),
            "Horizontal" => Ok(Self::Horizontal),
            "Vertical" => Ok(Self::Vertical),
            "ThreeLeft" => Ok(Self::ThreeLeft),
            "ThreeTop" => Ok(Self::ThreeTop),
            "ThreeRight" => Ok(Self::ThreeRight),
            "ThreeBottom" => Ok(Self::ThreeBottom),
            _ => Err(())
        }
    }

}

pub enum LayoutProperty {
    Width(i32),
    Height(i32),
    HorizontalRatio(f64),
    VerticalRatio(f64),
    Split(GroupSplit)
}

pub enum DesignProperty {
    BackgroundColor(String),
    GridColor(String),
    GridWidth(i32),
    Font(String)
}

pub enum ScaleProperty {
    Label(String),
    Min(f64),
    Max(f64),
    Log(bool),
    Invert(bool),
    GridOffset(i32),
    Precision(i32),
    NIntervals(i32),
    Adjustment(Adjustment)
}

pub enum LineProperty {
    Color(String),
    Width(f64),
    Dash(i32),
    X(Vec<f64>),
    Y(Vec<f64>)
}

pub enum ScatterProperty {
    Color(String),
    Radius(f64),
    X(Vec<f64>),
    Y(Vec<f64>)
}

pub enum TextProperty {
    Color(String),
    Font(String),
    X(Vec<f64>),
    Y(Vec<f64>),
    Text(Vec<String>)
}

/// Must come with plot position and mapping position within plot
pub enum MappingProperty {
    Line(LineProperty),
    Scatter(ScatterProperty),
    Text(TextProperty)
}

pub enum ScaleMode {
    Horizontal,
    Vertical
}

/// Must come with plot position
pub enum PlotProperty {

    Scale(ScaleMode, ScaleProperty),

    // Carries mapping position and property
    Mapping(usize, MappingProperty)
}

pub enum GroupProperty {

    Layout(LayoutProperty),

    Design(DesignProperty),

    // Carries plot position and property
    Plot(usize, PlotProperty)
}

// Maybe study using strong_xml crate?

/// A Panel has at least one first plot. //
#[derive(Clone)]
pub struct Panel {

    design : PlotDesign,

    plots : Vec<Plot>,

    split : GroupSplit,

    h_ratio : f64,

    v_ratio : f64,

    dimensions : (usize, usize),

    // parser : Parser,

    doc : Document
}

impl Default for Panel {

    fn default() -> Self {
        Self {
            design : Default::default(),
            plots : vec![Plot::default()],
            split : GroupSplit::Unique,
            h_ratio : 0.5,
            v_ratio : 0.5,
            dimensions : (800, 600),
            doc : Document::new().unwrap()
        }
    }

}

impl fmt::Debug for Panel {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{{ design : {:?}, plots : {:?}, split : {:?}, h_ratio : {:?}, v_ratio : {:?}, dimensions : {:?} }}",
            self.design,
            self.plots,
            self.split,
            self.h_ratio,
            self.v_ratio,
            self.dimensions
        )
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical
}

impl Panel {

    pub fn single(p1 : Plot) -> Self {
        let mut panel = Self::default();
        panel.plots[0] = p1;
        panel
    }

    pub fn pair(orientation : Orientation, p1 : Plot, p2 : Plot) -> Self {
        let mut group = Self::default();
        group.plots.push(Plot::default());
        group.split = match orientation {
            Orientation::Vertical => GroupSplit::Vertical,
            Orientation::Horizontal => GroupSplit::Horizontal
        };
        group.plots[0] = p1;
        group.plots[1] = p2;
        group
    }

    pub fn update(&mut self, prop : GroupProperty) {
        match prop {
            GroupProperty::Layout(layout) => {
                match layout {
                    LayoutProperty::Split(split) => { self.split = split },
                    LayoutProperty::VerticalRatio(vr) => { self.v_ratio = vr },
                    LayoutProperty::HorizontalRatio(hr) => { self.h_ratio = hr },
                    LayoutProperty::Width(w) => { self.dimensions.0 = w as usize },
                    LayoutProperty::Height(h) => { self.dimensions.1 = h as usize },
                }
            },
            GroupProperty::Design(design) => {
                match design {
                    DesignProperty::BackgroundColor(color) => { self.design.bg_color = color.parse().unwrap() },
                    DesignProperty::GridColor(color) => { self.design.grid_color = color.parse().unwrap() },
                    DesignProperty::GridWidth(w) => { self.design.grid_width = w },
                    DesignProperty::Font(f) => { self.design.font = FontData::new_from_string(&f[..]) }
                }
            },
            GroupProperty::Plot(ix, prop) => {
                self.plots[ix].update(prop);
            }
        }
    }

    pub fn to_json(&self) -> String {
        unimplemented!()
    }

    /*pub fn update_panel_directly(&mut self, prop : &str, val : &str) {
        match prop {
            "split" => { self.split = GroupSplit::from_str(val).unwrap() },
            "vertical_ratio" => { self.v_ratio = f64::from_str(val).unwrap() },
            "horizontal_ratio" => { self.h_ratio = f64::from_str(val).unwrap() },
            _ => panic!("Unrecognized panel property")
        }
    }*/

    pub fn new_from_json(json : &str) -> Result<Self, String> {

        // println!("Received JSON: {:?}", json);

        let opt_panel : Option<json::Panel> = serde_json::from_str(json).ok();
        if let Some(panel) = opt_panel {
            unimplemented!()
        } else {
            let mut plot : json::Plot = serde_json::from_str(json)
                .map_err(|e| format!("Error parsing plot = {}", e) )?;
            let design_json = plot.design.clone().unwrap_or_default();
            let layout_json = plot.layout.clone().unwrap_or_default();
            let design = PlotDesign::new_from_json(design_json)
                .map_err(|e| format!("Error parsing design = {}", e))?;
            let area = Plot::new_from_json(plot)
                .map_err(|e| format!("Error parsing area from JSON definition = {}", e) )?;
            Ok(Self {
                design,
                plots : vec![area],
                split : GroupSplit::Unique,
                h_ratio : layout_json.horizontal_ratio,
                v_ratio : layout_json.vertical_ratio,
                dimensions : (layout_json.width as usize, layout_json.height as usize),
                doc : Document::new().unwrap()
            })
        }
    }

    pub fn new(layout_path : String) -> Result<Self, String> {
        let plots = Vec::new();
        let mut parser : Parser = Default::default();
        let doc = parser.parse_file(&layout_path)
            .map_err(|e| format!("Failed parsing XML file: {}", e) )?;
        let root = doc.get_root_element().ok_or(format!("No root node"))
            .map_err(|_| format!("No root node"))?;

        let design_node = root
            .findnodes("object[@class='design']")
            .ok()
            .and_then(|nodes| nodes.first().cloned() )
            .ok_or(format!("No design node"))?;
        let design = PlotDesign::new(&design_node)
            .map_err(|e| format!("Failed instantiating design: {}", e))?;

        let dim_node = root
            .findnodes("object[@class='dimensions']")
            .ok()
            .and_then(|nodes| nodes.first().cloned() )
            .ok_or(format!("No dimensions node"))?;
        let dims = utils::children_as_hash(&dim_node, "property");

        let width = dims.get("width")
            .and_then(|w| w.parse::<usize>().ok() )
            .ok_or(format!("Missing width property"))?;
        let height = dims.get("height")
            .and_then(|h| h.parse::<usize>().ok() )
            .ok_or(format!("Missing height property"))?;
        let mut plot_group = Self {
            // parser,
            doc,
            plots,
            split : GroupSplit::Unique,
            v_ratio : 0.5,
            h_ratio : 0.5,
            design,
            dimensions : (width, height)
        };
        plot_group.load_layout(layout_path)?;
        Ok(plot_group)
    }

    /*pub fn update_text_mapping_with_adjustment(&mut self, active : usize, key : &str, pos : Vec<Vec<f64>>, text : Vec<String>, adj : Adjustment) {
        match self.update_mapping(active, &key, &pos) {
            Err(e) => { println!("Error updating text mapping: {}", e); },
            _ => {
                if let Err(e) = self.update_mapping_text(active, &key, &text) {
                    println!("Error adding text to mapping: {}", e);
                }
            }
        }
        self.adjust_scales(active, adj, adj);
    }

    pub fn update_mapping_with_adjustment(&mut self, active : usize, key : &str, data : Vec<Vec<f64>>, adj : Adjustment) {
        if let Err(e) = self.update_mapping(active, &key, &data) {
            println!("Error updating mapping {:}: {}", key, e);
        }
        self.adjust_scales(active, adj, adj);
    }*/

    /*/// Adjust scales so they fit the current data
    pub fn adjust_scales(&mut self, active : usize, adj_x : Adjustment, adj_y : Adjustment) {
        if let Some(((new_xmin, new_xmax), (new_ymin, new_ymax))) = self.data_limits(active) {
            context_mapper::adjust_segment(&mut self.plots[active].x, adj_x, new_xmin, new_xmax);
            context_mapper::adjust_segment(&mut self.plots[active].y, adj_y, new_ymin, new_ymax);
        } else {
            println!("Could not retrieve data limits");
        }
    }*/
    pub fn adjust_scales(&mut self) {
        self.plots.iter_mut().for_each(|pl| pl.adjust_scales() );
    }

    pub fn set_dimensions(&mut self, opt_w : Option<usize>, opt_h : Option<usize>) {
        let root = self.doc.get_root_element().unwrap();
        let dim_node = root
            .findnodes("object[@class='dimensions']")
            .expect("No dimensions node")
            .first()
            .cloned()
            .expect("No dimensions node");
        let set_new = |node : &Node, name : &str, value : usize| {
            // println!("Searching for {}", name);
            // println!("At {:?}", node.get_child_nodes().iter().map(|c| c.get_property("name")).collect::<Vec<_>>() );
            match node.findnodes(&format!("property[@name='{}']", name)) {
                Ok(mut props) => {
                    if let Some(p) = props.iter_mut().next() {
                        if let Err(e) = p.set_content(&(value.to_string())) {
                            println!("Error setting node content: {}", e);
                            return;
                        }
                    } else {
                        println!("No property named {} found", name);
                    }
                },
                _ => { println!("Failed at finding property {}", name); }
            }
        };

        if let Some(w) = opt_w {
            self.dimensions.0 = w;
            set_new(&dim_node, "width", w);
        }
        if let Some(h) = opt_h {
            self.dimensions.1 = h;
            set_new(&dim_node, "height", h);
        }
    }

    pub fn clear_all_data(&mut self) {
        for area in self.plots.iter_mut() {
            area.clear_all_data();
        }
    }

    pub fn svg(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buf : Vec<u8> = Vec::new();
        let surf = SvgSurface::for_stream(
            self.dimensions.0 as f64,
            self.dimensions.1 as f64,
            buf
        ).map_err(|e| format!("Error creating SVG surface: {}", e) )?;
        let ctx = Context::new(&surf);
        self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);
        let stream = surf.finish_output_stream().unwrap();
        Ok(String::from_utf8(stream.downcast_ref::<Vec<u8>>().unwrap().clone())?)
    }

    pub fn draw_to_file(&mut self, path : &str) -> Result<(), String> {
        // TODO Error creating SVG surface: "error while writing to output stream

        let path = Path::new(path);
        if !path.parent().map(|par| par.exists() ).unwrap_or(false) {
            return Err(format!("Parent directory for image path {} does not exists", path.to_str().unwrap()));
        }

        match path.extension().and_then(|e| e.to_str() ) {
            Some("svg") => {
                let surf = SvgSurface::new(
                    self.dimensions.0 as f64,
                    self.dimensions.1 as f64,
                    Some(path)
                ).map_err(|e| format!("Error creating SVG surface: {}", e) )?;
                let ctx = Context::new(&surf);
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);
            },
            Some("png") => {
                let surf = ImageSurface::create(
                    Format::ARgb32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                ).map_err(|e| format!("Error creating PNG image surface: {}", e) )?;
                let ctx = Context::new(&surf);
                // ctx.scale(3.0, 3.0);
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);
                let mut f = File::create(path).map_err(|e| format!("Unable to open PNG file:{}", e))?;
                surf.write_to_png(&mut f)
                    .map_err(|e| format!("Error writing content to png: {}", e) )?;
            },
            Some("eps") => {
                let surf = PsSurface::new(
                    self.dimensions.0 as f64,
                    self.dimensions.1 as f64,
                    path
                ).map_err(|e| format!("Error creating Postscript surface: {}", e) )?;
                surf.set_eps(true);
                let ctx = Context::new(&surf);
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);
            },
            Some(other) => {
                return Err(format!("Invalid image export extension: {}", other));
            },
            None => {
                return Err(format!("No valid extension informed for image export file"));
            }
        };
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.plots.len()
    }

    /// Draws the current Plot definition to a Cairo context.
    /// Used internally by PlotView to draw to the context
    /// of a gtk::DrawingArea. Users can also retrive the context
    /// from cairo::ImageSurface::create() to plot directly to
    /// SVG/PNG/PDF files.
    pub fn draw_to_context(
        &mut self,
        ctx : &Context,
        x : i32,
        y : i32,
        w : i32,
        h : i32
    ) {
        let top_left = (0.05, 0.05);
        let top_right = (w as f64 * self.h_ratio, 0.05);
        let bottom_left = (0.05, h as f64 * self.v_ratio);
        let bottom_right = (w as f64 * self.h_ratio, h as f64 * self.v_ratio);
        for (i, plot) in self.plots.iter_mut().enumerate() {
            let origin_offset = match (&self.split, i) {
                (GroupSplit::Horizontal, 1) => top_right,
                (GroupSplit::Vertical, 1) => bottom_left,
                (GroupSplit::Four, 1) => top_right,
                (GroupSplit::Four, 2) => bottom_left,
                (GroupSplit::Four, 3) => bottom_right,
                (GroupSplit::ThreeLeft, 1) => top_right,
                (GroupSplit::ThreeLeft, 2) => bottom_right,
                (GroupSplit::ThreeTop, 1) => bottom_left,
                (GroupSplit::ThreeTop, 2) => bottom_right,
                (GroupSplit::ThreeRight, 0) => top_left,
                (GroupSplit::ThreeRight, 1) => top_right,
                (GroupSplit::ThreeRight, 2) => bottom_left,
                (GroupSplit::ThreeBottom, 0) => top_left,
                (GroupSplit::ThreeBottom, 1) => top_right,
                (GroupSplit::ThreeBottom, 2) => bottom_left,
                _ => top_left
            };

            let h_full_v = (1., self.v_ratio);
            let h_full_v_compl = (1., 1. - self.v_ratio);
            let h_v_full = (self.h_ratio, 1.);
            let h_compl_v_full = (1. - self.h_ratio, 1.);
            let h_compl_v = (1. - self.h_ratio, self.v_ratio);
            let h_v_compl = (self.h_ratio, 1. - self.v_ratio);
            let diag = (self.h_ratio, self.v_ratio);
            let diag_compl = (1. - self.h_ratio, 1. - self.v_ratio);
            let scale_factor = match (&self.split, i) {
                (GroupSplit::Horizontal, 0) => h_v_full,
                (GroupSplit::Horizontal, 1) => h_compl_v_full,
                /*(GroupSplit::Vertical, 0) => h_v_full,
                (GroupSplit::Vertical, 1) => h_compl_v_full,*/
                (GroupSplit::Vertical, 0) => h_full_v,
                (GroupSplit::Vertical, 1) => h_full_v_compl,
                (GroupSplit::Four, 0) => diag,
                (GroupSplit::Four, 1) => h_compl_v,
                (GroupSplit::Four, 2) => h_v_compl,
                (GroupSplit::Four, 3) => diag_compl,
                (GroupSplit::ThreeLeft, 0) => h_v_full,
                (GroupSplit::ThreeLeft, 1) => h_compl_v,
                (GroupSplit::ThreeLeft, 2) => diag_compl,
                (GroupSplit::ThreeTop, 0) => h_full_v,
                (GroupSplit::ThreeTop, 1) => h_v_compl,
                (GroupSplit::ThreeTop, 2) => diag_compl,
                (GroupSplit::ThreeRight, 0) => diag,
                (GroupSplit::ThreeRight, 1) => h_compl_v_full,
                (GroupSplit::ThreeRight, 2) => h_v_compl,
                (GroupSplit::ThreeBottom, 0) => diag,
                (GroupSplit::ThreeBottom, 1) => h_compl_v,
                (GroupSplit::ThreeBottom, 2) => h_full_v_compl,
                _ => (1., 1.)
            };
            let origin = (x as f64 + origin_offset.0, y as f64 + origin_offset.1);
            let size = ((w as f64 * scale_factor.0) as i32, (h as f64 * scale_factor.1) as i32);
            ctx.save();
            // ctx.move_to(0.0, 0.0);
            ctx.translate(origin.0, origin.1);
            // println!("i: {}; origin: {:?}, size: {:?}", i, origin, size);
            plot.draw_plot(&ctx, &self.design, size.0, size.1);
            ctx.restore();
        }
        //println!("--");
    }

    pub fn reload_layout_data(&mut self) -> Result<(), Box<dyn Error>> {
        let _root_el = self.doc.get_root_element()
            .expect("Root node not found");
        // for plot in self.plots.iter_mut() {
        //    plot.reload_layout_node( /*node.clone()*/ )?;
        // }
        Ok(())
    }

    pub fn update_after_parsed_content(&mut self) -> Result<(), String> {

        use GroupSplit::*;

        let root_el = self.doc.get_root_element()
            .ok_or(format!("Root node not found"))?;
        if &root_el.get_name()[..] != "Panel" {
            return Err(format!("Root node should be called Panel"));
        }
        self.plots.clear();
        let mut found_split = false;
        for node in root_el.get_child_nodes() {
            //println!("Node name: {}", node.get_name());
            if &node.get_name()[..] == "property" {
                //println!("Property: {:?}", node.get_attribute("name"));
                match node.get_attribute("name").as_ref().and_then(|s| Some(&s[..]) ) {
                    Some("vertical_ratio") => {
                        self.v_ratio = node.get_content().parse()
                            .map_err(|_| format!("Unabe to parse vertical ratio"))?;
                        // println!("v ratio set to {}", self.v_ratio);
                    },
                    Some("horizontal_ratio") => {
                        self.h_ratio = node.get_content().parse()
                            .map_err(|_| format!("Unabe to parse horizontal ratio"))?;
                        // println!("h ratio set to {}", self.h_ratio);
                    },
                    Some("split") => {
                        found_split = true;
                        match GroupSplit::from_str(&node.get_content()[..]) {
                            Ok(split) => {
                                self.split = split;
                            },
                            Err(_) => {
                                return Err(String::from("Unrecognized split value"));
                            }
                        }
                    },
                    _ => return Err(String::from("Unknown property"))
                }
            }
            if &node.get_name()[..] == "Plot" {
                self.plots.push(Plot::new_from_node(node.clone()));
            }
        }
        if self.plots.len() == 0 {
            return Err("Root node Panel does not contain any Plot children.".into());
        }
        if !found_split {
            self.split = Unique;
        }
        match self.split {
            Unique => if self.plots.len() != 1 {
                return Err("'None' split require 1 plot".into());
            },
            Vertical => if self.plots.len() != 2 {
                return Err("Vertical split require 2 plots".into());
            },
            Horizontal => if self.plots.len() != 2 {
                return Err("Horizontal split require 2 plots".into());
            },
            Four => if self.plots.len() != 4 {
                return Err("'Both' split require 4 plots".into());
            },
            ThreeLeft | ThreeTop | ThreeRight | ThreeBottom => if self.plots.len() != 3 {
                return Err("'Three' split require 3 plots".into());
            }
        }
        // self.reload_layout_data()
        //    .map_err(|e| format!("Could not reload layout data: {}", e))?;
        for plot in self.plots.iter_mut() {
            // plot.reload_mappings()
            //    .map_err(|e| format!("Could not reload mappings from informed layout: {}", e))?;
        }
        // println!("h: {}; v : {}; split: {:?}", self.h_ratio, self.v_ratio, self.split);
        Ok(())
    }

    pub fn load_layout_from_string(&mut self, content : String) -> Result<(), String> {
        let mut parser : Parser = Default::default();
        self.doc = parser.parse_string(content)
            .map_err(|e| format!("Failed parsing XML: {}", e) )?;
        self.update_after_parsed_content()
    }

    pub fn load_layout(&mut self, path : String) -> Result<(), String> {
        // TODO falling here when closing connection to SQLite database
        let mut parser : Parser = Default::default();
        self.doc = parser.parse_file(&path)
            .map_err(|e| format!("Failed parsing XML: {}", e) )?;
        self.update_after_parsed_content()
    }

    pub fn save_layout(&self, path : String) {
        // let content = self.get_layout_as_text();
        match File::create(path) {
            Ok(mut f) => {
                /*if let Err(e) = f.write_all(content.as_bytes()) {
                    println!("Error writing to file: {}", e);
                }*/
                let mut options : SaveOptions = Default::default();
                options.format = true;
                options.non_significant_whitespace = true;
                f.write_all(self.doc.to_string_with_options(options).as_bytes())
                    .map_err(|e| { println!("{}", e) });
            },
            Err(e) => println!("Error creating file: {}", e)
        }
        //self.doc.save_file(&path)
        //    .expect("Could not save file");
    }

    pub fn get_layout_as_text(&self) -> String {
        let mut opts : SaveOptions = Default::default();
        opts.format = true;
        opts.non_significant_whitespace = true;
        //self.doc.to_string(opts)
        self.doc.to_string_with_options(opts)
    }

    /*pub fn update_design_directly(&mut self, prop : &str, val : &str) {
        /*match prop {
            "bg_color" =>
            "grid_color"
            "grid_width"
            "font"
        }*/
    }*/

    pub fn update_design(&mut self, property : &str, value : &str) {
        // println!("Updating design at {} to {}", property, value);
        if property.is_empty() || value.is_empty() {
            // println!("Informed empty property!");
            return;
        }
        let root = self.doc.get_root_element().unwrap();
        let design_node = root
            .findnodes("object[@class='design']")
            .expect("No design node")
            .first().cloned().expect("No design node");
        match design_node.findnodes(&property) {
            Ok(mut props) => {
                if let Some(p) = props.iter_mut().next() {
                    if let Err(e) = p.set_content(&value) {
                        println!("Error setting node content: {}", e);
                        return;
                    }
                    self.design = PlotDesign::new(&design_node)
                        .expect("Failed loading plot design");
                } else {
                    println!("No property named {} found", property);
                }
            },
            _ => { println!("Failed at finding property {}", property); }
        }
    }

    pub fn update_plot_property(&mut self, ix: usize, property : &str, value : &str) {
        // println!("Updating {} at {} to {}", ix, property, value);
        if let Err(e) = self.plots[ix].update_layout(property, value) {
            println!("{}", e);
        }
    }

    pub fn update_mapping(&mut self, ix : usize, id : &str, data : &Vec<Vec<f64>>) -> Result<(), Box<dyn Error>> {
        // println!("Updating {} at {} to {:?}", ix, id, data);
        self.plots[ix].update_mapping(id, data)
    }

    pub fn update_mapping_text(&mut self, ix : usize, id : &str, text : &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.plots[ix].update_mapping_text(id, text)
    }

    pub fn update_mapping_columns(&mut self, ix : usize, id : &str, cols : Vec<String>) -> Result<(), Box<dyn Error>> {
        self.plots[ix].update_mapping_columns(id, cols)
    }

    pub fn update_source(&mut self, ix : usize, id : &str, source : String) -> Result<(), Box<dyn Error>> {
        self.plots[ix].update_source(id, source)
    }

    pub fn add_mapping(
        &mut self,
        ix : usize,
        mapping_index : String,
        mapping_type : String,
        mapping_source : String,
        col_names : Vec<String>
    ) -> Result<(), String> {
        self.plots[ix].add_mapping(mapping_index, mapping_type, mapping_source, col_names, &self.doc)
    }

    pub fn ordered_col_names(&self, ix : usize, id : &str) -> Vec<(String, String)> {
        self.plots[ix].mapping_column_names(id)
    }

    pub fn remove_mapping(&mut self, ix : usize, id : &str) {
        if let Err(e) = self.plots[ix].remove_mapping(id) {
            println!("{}", e);
        }
    }

    pub fn scale_info(&self, ix : usize, scale : &str) -> HashMap<String, String> {
        self.plots[ix].scale_info(scale)
    }

    pub fn design_info(&self) -> HashMap<String, String> {
        self.design.description()
    }

    pub fn mapping_info(&self, ix : usize) -> Vec<(String, String, HashMap<String,String>)> {
        self.plots[ix].mapping_info()
    }

    pub fn group_split(&self) -> GroupSplit {
        self.split.clone()
    }

    pub fn aspect_ratio(&self) -> (f64, f64) {
        (self.h_ratio, self.v_ratio)
    }

    pub fn data_limits(&self, ix : usize) -> Option<((f64, f64), (f64, f64))> {
        self.plots[ix].max_data_limits()
    }

    pub fn set_aspect_ratio(&mut self, horiz : Option<f64>, vert : Option<f64>) {
        if let Some(horiz) = horiz {
            self.h_ratio = horiz;
        }
        if let Some(vert) = vert {
            self.v_ratio = vert;
        }
    }

    pub fn reassign_plot(
        &mut self,
        src_plot : usize,
        src_ix : &str,
        dst_plot : usize
    ) -> Result<(), String> {
        let (mapping, mut mapping_node) = if let Some(mut old_area) = self.plots.get_mut(src_plot) {
            match old_area.remove_mapping(src_ix) {
                Ok((mapping, mapping_node)) => (mapping, mapping_node),
                Err(e) => return Err(format!("{}",e))
            }
        } else {
            return Err(format!("Invalid source plot"));
        };
        if let Some(mut new_area) = self.plots.get_mut(dst_plot) {
            new_area.node.add_child(&mut mapping_node).map_err(|e| format!("{}", e) )?;
            new_area.mappings.push(mapping);
            Ok(())
        } else {
            Err(format!("Invalid destination plot"))
        }
    }

    // Number of mappings, for each plot
    pub fn n_mappings(&self) -> Vec<usize> {
        self.plots.iter().map(|p| p.mappings.len() ).collect()
    }

    // Number of plots
    pub fn n_plots(&self) -> usize {
        self.plots.len()
    }

    pub fn view_all_sources(&self) -> Vec<String> {
        self.plots.iter().map(|plot| plot.view_sources() ).flatten().collect()
    }

    pub fn view_grouped_sources(&self) -> Option<String> {
        let sources = self.view_all_sources();
        let mut sql_text = String::new();
        for source in sources {
            sql_text += &format!("\n{}\n", source)[..];
        }
        if !sql_text.is_empty() {
            Some(sql_text)
        } else {
            None
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.dimensions.0 as u32, self.dimensions.1 as u32)
    }

}

#[derive(Clone, Debug)]
pub struct Plot {
    mappings : Vec<Box<dyn Mapping>>,
    mapper : ContextMapper,
    x : Scale,
    y : Scale,
    frozen : bool,
    node : Node
}

impl Default for Plot {
    fn default() -> Self {
        let mappings = Vec::new();
        let mapper : ContextMapper = Default::default();
        let x : Scale = Default::default();
        let y : Scale = Default::default();
        let frozen = false;
        Plot{ mappings, mapper, x, y, frozen, node : Node::null() }
    }
}

#[derive(Debug)]
pub enum PlotError {
    InvalidData(&'static str),
    OutOfBounds(&'static str),
    Other(&'static str),
    Parsing
}

impl PlotError {
    pub fn new() -> Self {
        Self::Other("Unknown error")
    }
}

impl Display for PlotError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidData(msg) => { write!(f, "{}", msg) },
            Self::OutOfBounds(msg) => { write!(f, "{}", msg) },
            Self::Other(msg) => { write!(f, "{}", msg) }
            Self::Parsing => { write!(f, "Parsing error") }
        }
    }

}

impl error::Error for PlotError {

}

impl showable::Show for Plot {
    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Panel::single(self.clone()).svg().unwrap())
    }

    fn modality(&self) -> showable::Modality {
        showable::Modality::XML
    }
}

impl showable::Show for Panel {
    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone().svg().unwrap())
    }

    fn modality(&self) -> showable::Modality {
        showable::Modality::XML
    }
}

impl Plot {

    pub fn svg(&self) -> String {
        Panel::single(self.clone()).svg().unwrap()
    }

    pub fn scale_x(mut self, scale : Scale) -> Self {
        self.x = scale;
        self.adjust_scales();
        self
    }

    pub fn scale_y(mut self, scale : Scale) -> Self {
        self.y = scale;
        self.adjust_scales();
        self
    }

    pub fn draw(mut self, map : impl Mapping + 'static) -> Self {
        self.mappings.push(Box::new(map) as Box<dyn Mapping>);
        self.adjust_scales();
        self
    }

    pub fn update(&mut self, prop : PlotProperty) {
        match prop {
            PlotProperty::Scale(mode, prop) => {
                match mode {
                    ScaleMode::Horizontal => self.x.update(prop),
                    ScaleMode::Vertical => self.y.update(prop)
                }
            },
            PlotProperty::Mapping(ix, m) => {
                if !self.mappings[ix].update(m) {
                    panic!("Could not update mapping");
                }
            }
        }
        self.adjust_scales();
    }

    pub fn adjust_scales(&mut self) {
        if let Some(((new_xmin, new_xmax), (new_ymin, new_ymax))) = self.max_data_limits() {
            println!("Data limits = {:?}", (new_xmin, new_xmax, new_ymin, new_ymax));
            let (x_adj, y_adj) = (self.x.adj, self.y.adj);
            println!("Adjustments = {:?}", (x_adj, y_adj));
            scale::adjust_segment(&mut self.x, x_adj, new_xmin, new_xmax);
            scale::adjust_segment(&mut self.y, y_adj, new_ymin, new_ymax);
            self.mapper.update_data_extensions(self.x.from, self.x.to, self.y.from, self.y.to);
        } else {
            println!("Could not retrieve data limits");
        }
    }

    pub fn new_from_json(mut rep : json::Plot) -> Result<Plot, Box<dyn Error>> {

        // println!("JSON rep: {:?}", rep);

        let mut mappings = Vec::new();

        // println!("Received mappings: {:?}", rep.mappings);
        for mapping in rep.mappings.iter_mut() {
            mappings.push(mappings::new_from_json(mem::take(mapping))?);
        }

        let x = Scale::new_from_json(rep.x.clone()).ok_or(PlotError::Parsing)?;
        let y = Scale::new_from_json(rep.y.clone()).ok_or(PlotError::Parsing)?;

        let mapper = ContextMapper::new(
            x.from,
            x.to,
            y.from,
            y.to,
            x.log,
            y.log,
            x.invert,
            y.invert
        );

        let mut area = Self {
            mappings,
            mapper,
            x,
            y,
            frozen : false,
            node : Node::null()
        };
        area.adjust_scales();

        println!("Plot area = {:?}", area);

        // We do not load any design definitions here, but rather at Panel::new(),
        // since the design might be defined at Panel-level.
        Ok(area)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from_node(node : Node) -> Plot {

        // if let Err(e) = pl_area.reload_layout_data() {
        //   println!("Error when reloading layout data: {}", e.description());
        // }
        Default::default()
    }

    /*fn save(&self, path : String, w : i32, h : i32) {

    }*/

    fn draw_plot(&mut self, ctx: &Context, design : &PlotDesign, w : i32, h : i32) {
        self.mapper.update_dimensions(w, h);
        // If frozen, do not redraw background/grid.
        // Draw only frozen mapping increment.
        self.draw_background(ctx, design);
        self.draw_grid(ctx, design);
        for mapping in self.mappings.iter() {
            // println!("Mapping drawn");
            mapping.draw(&self.mapper, &ctx);
        }
    }

    //fn on_draw(&self, ctx : &Context) {
    //}

    pub fn max_data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let mut x_lims = Vec::new();
        let mut y_lims = Vec::new();
        for (xl, yl) in self.mappings.iter().filter_map(|m| m.data_limits() ) {
            x_lims.push(xl);
            y_lims.push(yl);
        }
        let min_x = x_lims.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal) )?.0;
        let max_x = x_lims.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal) )?.1;
        let min_y = y_lims.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal) )?.0;
        let max_y = y_lims.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal) )?.1;
        Some(((min_x, max_x), (min_y, max_y)))
    }

    pub fn freeze_at_mapping(&mut self, _mapping : &str) -> Result<(),()> {
        // Call mapping.setup
        // set frozen to true
        Err(())
    }

    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    // let props = ["from", "to", "n_intervals", "invert", "log_scaling"];

    // TODO improve error handling here.
    fn read_grid_segment(
        &self,
        props : HashMap<String, String>
    ) -> Result<Scale, Box<dyn Error>> {
        let from : f64 = props.get("from").unwrap().parse()?;
        let to : f64 = props["to"].parse()?;
        let nint : i32 = props["n_intervals"].parse()?;
        let offset : i32 = props["grid_offset"].parse()?;
        let invert : bool = props["invert"].parse()?;
        let log : bool = props["log_scaling"].parse()?;
        let precision : i32 = props["precision"].parse()?;
        let label = props["label"].clone();
        Ok( Scale::new_full(
            label, precision, from, to, nint, log, invert, offset, Adjustment::Off) )
    }

    /// Reloads all mappings from XML definition,
    /// clearing any existent data.
    pub fn reload_mappings(&mut self) -> Result<(),String> {
        // let root = self.doc.get_root_element()
        //    .expect("Root node not found");
        self.mappings.clear();
        if let Ok(mappings) = self.node.findnodes("object[@class='mapping']") {
            //println!("mappings to add -> {:?}", mappings);
            for (i, mapping_node) in mappings.iter().enumerate() {
                let mapping_ix = mapping_node
                    .get_attribute("index")
                    .ok_or(format!("Missing 'index' attribute for mapping at position {}", i))?;
                let mapping_type = mapping_node
                    .get_attribute("type")
                    .ok_or(format!("Missing 'type' attribute for mapping {}", mapping_ix))?;
                let mapping : Option<Box<dyn Mapping>> = match &mapping_type[..] {
                    "line" => Some( Box::new(LineMapping::new(&mapping_node)?) ),
                    "scatter" => Some( Box::new(ScatterMapping::new(&mapping_node)?) ),
                    "bar" => Some( Box::new(BarMapping::new(&mapping_node)?) ),
                    "text" => Some( Box::new(TextMapping::new(&mapping_node)?) ),
                    "area" => Some( Box::new(AreaMapping::new(&mapping_node)?) ),
                    "surface" => Some( Box::new(SurfaceMapping::new(&mapping_node)?) ),
                    _ => None
                };
                if let Some(m) = mapping{
                    self.mappings.insert(mapping_ix.parse::<usize>().unwrap(), m);
                } else {
                    return Err(format!("Unrecognized mapping type: {}", mapping_type));
                }
            }
            Ok(())
        } else {
            Err(format!("Error finding mapping nodes"))
        }
    }

    /// Parses the XML file definition at self.doc
    /// and updates all layout information used for plotting.
    /// Does not mess with mapping data.
    pub fn reload_layout_node(&mut self /*, node : Node*/ ) -> Result<(), Box<dyn Error>> {
        // TODO confirm this does not need to be reset here.
        // self.node = node;
        // let root = self.doc.get_root_element()
        //    .expect("Root node not found");
        // println!("updating node: {:?} position: {:?}", self.node.get_name(), self.node.get_property("position"));
        let xprops = utils::children_as_hash(
            &self.node, "object[@name='x']/property");
        // println!("xprops: {:?}", xprops);
        let yprops = utils::children_as_hash(
            &self.node, "object[@name='y']/property");
        self.x = self.read_grid_segment(xprops)?;
        // println!("x grid: {:?}", self.x);
        self.y = self.read_grid_segment(yprops)?;
        self.mapper = ContextMapper::new(self.x.from, self.x.to,
            self.y.from, self.y.to, self.x.log, self.y.log,
            self.x.invert, self.y.invert);
        Ok(())
    }

    fn new_base_mapping_node(
        &self,
        mapping_type : &str,
        mapping_index : &str,
        doc : &Document
    ) -> Result<Node,Box<dyn Error>> {
        let mut new_mapping = Node::new("object", Option::None, &doc)
            .unwrap();
        new_mapping.set_attribute("class", "mapping")?;
        new_mapping.set_attribute("type", &mapping_type)?;
        new_mapping.set_attribute("index", &mapping_index)?;

        /*let mut color_prop = Node::new(
            "property", Option::None, &self.doc)
            .unwrap();
        color_prop.set_attribute("name", "color")?;
        color_prop.set_content("#000000")?;
        new_mapping.add_child(&mut color_prop)?;*/
        Ok(new_mapping)
    }

    pub fn add_mapping(
        &mut self,
        mapping_index : String,
        mapping_type : String,
        mapping_source : String,
        col_names : Vec<String>,
        doc : &Document
    ) -> Result<(), String> {
        //let mut root = self.doc.get_root_element().expect("No root");
        let mut new_mapping = self.new_base_mapping_node(
            &mapping_type[..],
            &mapping_index[..],
            &doc
        ).map_err(|e| format!("{}", e) )?;

        if let Some(mtype) = MappingType::from_str(&mapping_type[..]) {

            let mut mapping_data = mtype.default_hash();
            *(mapping_data.get_mut("source").unwrap()) = mapping_source.clone();

            utils::populate_node_with_hash(
                &doc,
                &mut new_mapping,
                mapping_data
            ).map_err(|e| format!("{}", e) )?;

            let m_ix : usize = mapping_index.parse::<usize>().map_err(|e| format!("{}", e) )?;
            if m_ix > self.mappings.len() {
                return Err(format!(
                    "Tried to insert mapping at position {}, but plot has only {} elements",
                    m_ix, self.mappings.len()
                ));
            }

            // println!("Received col names: {:?}", col_names);
            match mtype {
                MappingType::Line => {
                    let mut line_mapping = LineMapping::new(&new_mapping)?;
                    line_mapping.set_source(mapping_source);
                    line_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(line_mapping));
                },
                MappingType::Scatter => {
                    let mut scatter_mapping = ScatterMapping::new(&new_mapping)?;
                    scatter_mapping.set_source(mapping_source);
                    scatter_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(scatter_mapping));
                },
                MappingType::Bar => {
                    let mut bar_mapping = BarMapping::new(&new_mapping)?;
                    bar_mapping.set_source(mapping_source);
                    bar_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(bar_mapping));
                },
                MappingType::Text => {
                    let mut text_mapping = TextMapping::new(&new_mapping)?;
                    text_mapping.set_source(mapping_source);
                    text_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(text_mapping));
                },
                MappingType::Area => {
                    let mut area_mapping = AreaMapping::new(&new_mapping)?;
                    area_mapping.set_source(mapping_source);
                    area_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(area_mapping));
                },
                MappingType::Surface => {
                    let mut surface_mapping = SurfaceMapping::new(&new_mapping)?;
                    surface_mapping.set_source(mapping_source);
                    surface_mapping.set_col_names(col_names);
                    self.mappings.insert(m_ix, Box::new(surface_mapping));
                }
            }

            let updated_props = self.mappings[m_ix].properties();

            // println!("Updated mapping properties: {:?}", updated_props);

            utils::edit_node_with_hash(
                &doc,
                &updated_props,
                &mut new_mapping
            );
            self.node.add_child(&mut new_mapping)?;

            // TODO verify if this is necessary!!
            // self.doc.set_root_element(&root);
        } else {
            return Err(format!("Unrecognized mapping {}", mapping_type));
        }

        /*if mapping_type == "line" {
            let mtype = MappingType::Line;

            let mut width_property = Node::new(
                "property", Option::None, &self.doc).unwrap();
            let mut dash_property = Node::new(
                "property", Option::None, &self.doc).unwrap();
            width_property.set_attribute("name", "width")?;
            dash_property.set_attribute("name", "dash")?;
            width_property.set_content("1")?;
            dash_property.set_content("1")?;
            new_mapping.add_child(&mut width_property)?;
            new_mapping.add_child(&mut dash_property)?;
            let line_mapping = LineMapping::new(&new_mapping);
            println!("{:?}", mapping_name.clone());
            self.mappings.insert(mapping_name, Box::new(line_mapping));
            root.add_child(&mut new_mapping)?;
            self.doc.set_root_element(&root);
        }*/
        //if self.reload_layout_data().is_err() {
        //    println!("Problem reloading data after adding new mapping");
        //}

        Ok(())
    }

    fn accomodate_dimension(
        &mut self,
        data : &[f64],
        old_min : f64,
        old_max : f64,
        dim_name : &str
    ) {
        let new_min = data.iter().fold(old_min, |min, el| {
            if *el < min {
                *el
            } else {
                min
            }
        });
        let new_max = data.iter().fold(old_max, |max, el| {
            if *el > max {
                *el
            } else {
                max
            }
        });
        if new_min < old_min {
            let ans = self.update_layout(
                &format!("object[@name='{}']/property[@name='from']", dim_name)[..],
                &new_min.to_string()
            );
            if let Err(e) = ans {
                println!("{}", e);
            }
        }
        if new_max > old_max {
            let ans = self.update_layout(
                &format!("object[@name='{}']/property[@name='to']", dim_name)[..],
                &new_max.to_string()
            );
            if let Err(e) = ans {
                println!("{}", e);
            }
        }
    }

    pub fn update_mapping(
        &mut self,
        id : &str,
        data : &Vec<Vec<f64>>
    ) -> Result<(), Box<dyn Error>> {
        if data.len() < 1 {
            return Err(Box::new(PlotError::InvalidData("Invalid data")))
        }
        let (xmin, xmax, ymin, ymax) = self.mapper.data_extensions();
        if data.len() == 1 {
            self.accomodate_dimension(&data[0][..], ymin, ymax, "y");
        } else {
            self.accomodate_dimension(&data[0][..], xmin, xmax, "x");
            self.accomodate_dimension(&data[1][..], ymin, ymax, "y");
        }
        if let Some(mapping) = self.mappings.get_mut(id.parse::<usize>().unwrap()) {
            mapping.update_data(data.clone());
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Cannot recover mapping "
            )))
        }
    }

    pub fn remove_mapping(&mut self, id : &str) -> Result<(Box<dyn Mapping>, Node), String> {
        let n = self.mappings.len();
        let pos = id.parse::<usize>().map_err(|e| format!("Node id is not an integer: {}", id))?;
        //let mut root = self.doc.get_root_element().expect("No root at remove");
        let xpath = String::from("object[@index='") + id +  "']";
        // println!("Removing mapping at path: {}", xpath);
        let mut nodes = self.node.findnodes(&xpath[..])
            .map_err(|_| format!("No node with informed id: {}", id))?;
        let node = nodes.get_mut(0)
            .ok_or(format!("No first node with informed id: {}", id))?;
        node.unlink_node();
        let mapping = self.mappings.remove(pos);
        for i in (pos + 1)..n {
            let next_xpath = String::from("object[@index='") + &i.to_string()[..] +  "']";
            let mut nodes = self.node.findnodes(&next_xpath[..])
                .map_err(|e| format!("No next node with informed id: {}", i))?;

            // TODO error here at node removal
            let next_node = nodes.get_mut(0).ok_or(format!("No first node with informed id: {}", id))?;
            next_node.set_attribute("index", &((i - 1).to_string())[..])
                .map_err(|e| format!("Node {} missing index property", i));
        }
        self.reload_mappings()?;
        // for m in self.mappings.iter() {
        //    println!("Current remaining mappings: {:?}", m.mapping_type());
        // }
        // println!("Mapping {} removed successfully", id);
        Ok((mapping, node.clone()))
    }

    pub fn update_mapping_text(
        &mut self,
        id : &str,
        text : &Vec<String>
    ) -> Result<(), Box<dyn Error>> {
        if let Some(mapping) = self.mappings.get_mut(id.parse::<usize>().unwrap()) {
            mapping.update_extra_data(vec![text.clone()]);
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Unable to update text mapping position"
            )))
        }

        /*
            // println!("{}, {:?}", mapping.mapping_type(), mapping.properties());
            {
            // let mapping = mapping as &mut dyn Any;
            // println!("{:?}", (mapping as &mut dyn Any).type_id());
            match (mapping as &mut dyn Any).downcast_mut::<TextMapping>() {
                Some(m) => {
                    m.set_text_data(&text);
                    Ok(())
                },
                None => {
                    Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        "Informed mapping does not support text update"
                    )))
                }
            }
            }
        } else {
            Err(Box::new(std::io::Error::new(
                ErrorKind::Other, "Cannot recover mapping")))
        }*/
    }

    /* Given a resolvable full path to a property, update it. */
    pub fn update_layout(&mut self, property : &str, value : &str) -> Result<(), String> {
        // let root = self.doc.get_root_element().expect("No root");
        // println!("{} : {}", property, value);
        if property.is_empty() || value.is_empty() {
            return Err(format!("Informed empty property!"));
        }

        match self.node.findnodes(&property) {
            Ok(mut props) => {
                if let Some(p) = props.iter_mut().next() {
                    if let Err(e) = p.set_content(&value) {
                        println!("Error setting node content: {}", e);
                    }
                    // println!("new node content: {:?}, {:?}", p.get_property("name"), p.get_content());
                    // println!("new node at root: {:?}", self.node.get_content());
                    let parent = p.get_parent().unwrap();
                    match parent.get_attribute("class") {
                        Some(ref class) if class == "mapping" => {
                            if let Some(index) = parent.get_attribute("index") {
                                if let Some(m) = self.mappings.get_mut(index.parse::<usize>().unwrap()) {
                                    m.update_layout( &parent )?;
                                } else {
                                    println!("No mapping at {} available", index);
                                }
                            } else {
                                println!("Invalid mapping index");
                            }
                        },
                        Some(ref class) if class != "mapping" => {
                            //println!(
                            //    "Updated property: {:?}",
                            //    self.node.findnodes(property).unwrap().iter().next().unwrap().get_content()
                            //);
                            if let Err(e) = self.reload_layout_node() {
                                println!("Could not apply property {} ({})", property, e);
                            }
                            //println!(
                            //    "Updated property after reload: {:?}",
                            //    self.node.findnodes(property).unwrap().iter().next().unwrap().get_content()
                            //);
                        },
                        _ => {
                            println!("Layout item missing class attribute.");
                        }
                    }
                } else {
                    println!("{}", "Property ".to_owned() + property + " not found!");
                }
            },
            Err(e) => {
                println!("No property {} found at node {:?} ({:?})", property, self.node, e);
            }
        }
        Ok(())
    }

    pub fn clear_all_data(&mut self) {
        for m in self.mappings.iter_mut() {
            let mut empty_data : Vec<Vec<f64>> = Vec::new();
            match &m.mapping_type()[..] {
                "line" => {
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                },
                "scatter" => {
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                },
                "bar" => {
                    empty_data.push(Vec::new());
                },
                "area" => {
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                },
                "text" => {
                    //TODO clear text
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                    match (m as &mut dyn Any).downcast_mut::<TextMapping>() {
                        Some(m) => {
                            m.set_text_data(&Vec::new());
                        },
                        _ => { println!("Could not downcast to text when clearing its data"); }
                    }
                },
                "surface" => {
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                    empty_data.push(Vec::new());
                }
                _ => {
                    println!("Invalid mapping type");
                    return;
                }
            }
            m.update_data(empty_data);
        }
    }

    fn draw_background(&self, ctx : &Context, design : &PlotDesign) {
        ctx.save();
        ctx.set_line_width(0.0);
        ctx.set_source_rgb(
            design.bg_color.red,
            design.bg_color.green,
            design.bg_color.blue);
        ctx.rectangle(
            0.1*(self.mapper.w as f64), 0.1*(self.mapper.h as f64),
            0.8*(self.mapper.w as f64), 0.8*(self.mapper.h as f64));
        ctx.fill();
        ctx.restore();
    }

    fn draw_grid_line(
        &self,
        ctx : &Context,
        design : &PlotDesign,
        from : Coord2D,
        to : Coord2D
    ) {
        ctx.save();
        ctx.set_source_rgb(
            design.grid_color.red,
            design.grid_color.green,
            design.grid_color.blue);
        ctx.move_to(from.x, from.y);
        ctx.line_to(to.x, to.y);
        ctx.stroke();

        //ctx.set_source_rgb(0.2666, 0.2666, 0.2666);
        //ctx.move_to(from.x + label_off_x, from.y + label_off_y);
        //ctx.show_text(&label);
        //self.draw_centered_label(ctx, &label, Coord2D::new(from.x + label_off_x, from.y + label_off_y), false);
        //self.draw_grid_value(ctx, &label)
        ctx.restore();
    }

    /// Since the y value is always centered, this function accepts the option
    /// to center the x value (true for the x labels; false for the y labels).
    fn draw_grid_value(
        &self,
        ctx : &Context,
        design : &PlotDesign,
        value : &str,
        pos : Coord2D,
        center_x : bool,
        ext_off_x : f64,
        ext_off_y : f64
    ) {
        ctx.set_source_rgb(0.2666, 0.2666, 0.2666);
        text::draw_label(
            &design.font.sf,
            ctx,
            &value[..],
            pos,
            false,
            (center_x, true),
            Some(ext_off_x),
            Some(ext_off_y)
        );
    }

    pub fn steps_to_labels(
        steps : &[f64],
        precision : usize
    ) -> Vec<String> {
        steps.iter()
            .map(|s| format!("{:.*}", precision, s))
            .collect()
    }

    fn get_max_extent(
        &self,
        design : &PlotDesign,
        labels : &Vec<String>
    ) -> f64 {
        labels.iter()
            .map(|l| design.font.sf.text_extents(&l[..]).x_advance)
            .fold(0.0, |m, f| f64::max(m,f))
    }

    /*fn shift_coord_by_max_extent(
        base_coord : Coord2D,
        max_extent : f64
    ) -> Coord2D {

            .collect()
    }*/

    fn draw_grid(&self, ctx : &Context, design : &PlotDesign) {
        ctx.save();
        ctx.set_line_width(design.grid_width as f64);
        design.font.set_font_into_context(&ctx);
        let mut x_labels = Plot::steps_to_labels(
            &self.x.steps[..],
            self.x.precision as usize
        );
        if self.mapper.xinv {
            x_labels.reverse();
        }
        for (x, x_label) in self.x.steps.iter().zip(x_labels.iter()) {
            let from = match (self.mapper.xinv, self.mapper.yinv) {
                (false, false) => self.mapper.map(*x, self.mapper.ymin),
                (false, true) => self.mapper.map(*x, self.mapper.ymax),
                (true, false) => self.mapper.map(self.mapper.xmin + self.mapper.xmax - *x, self.mapper.ymin),
                (true, true) => self.mapper.map(self.mapper.xmin + self.mapper.xmax - *x, self.mapper.ymax)
            };
            let to = match (self.mapper.xinv, self.mapper.yinv) {
                (false, false) => self.mapper.map(*x, self.mapper.ymax),
                (false, true) => self.mapper.map(*x, self.mapper.ymin),
                (true, false) =>  self.mapper.map(self.mapper.xmin + self.mapper.xmax - *x, self.mapper.ymax),
                (true, true) => self.mapper.map(self.mapper.xmin + self.mapper.xmax - *x, self.mapper.ymin)
            };
            // let from = self.mapper.map(*x, self.mapper.ymin);
            // let to = match self.mapper.self.mapper.map(*x, self.mapper.ymax);
            // println!("{:?}, {:?}, {:?}", x, from, to);
            self.draw_grid_line(ctx, design, from, to);
            self.draw_grid_value(ctx, design, x_label, from, true, 0.0, 1.5);
        }

        let mut y_labels = Plot::steps_to_labels(
            &self.y.steps[..],
            self.y.precision as usize
        );
        if self.mapper.yinv {
            y_labels.reverse();
        }
        let max_extent = self.get_max_extent(design, &y_labels);
        for (y, y_label) in self.y.steps.iter().zip(y_labels.iter()) {
            let mut from = match (self.mapper.xinv, self.mapper.yinv) {
                (false, false) => self.mapper.map(self.mapper.xmin, *y),
                (false, true) => self.mapper.map(self.mapper.xmin, self.mapper.ymin + self.mapper.ymax - *y),
                (true, false) => self.mapper.map(self.mapper.xmax, *y),
                (true, true) => self.mapper.map(self.mapper.xmax, self.mapper.ymin + self.mapper.ymax - *y)
            };
            let to = match (self.mapper.xinv, self.mapper.yinv) {
                (false, false) => self.mapper.map(self.mapper.xmax, *y),
                (false, true) => self.mapper.map(self.mapper.xmax, self.mapper.ymin + self.mapper.ymax - *y),
                (true, false) =>  self.mapper.map(self.mapper.xmin, *y),
                (true, true) => self.mapper.map(self.mapper.xmin, self.mapper.ymin + self.mapper.ymax - *y)
            };
            self.draw_grid_line(ctx, design, from, to);
            //let mut y_label_coord = match self.mapper.yinv {
            //    true => to,
            //    false => from
            //};
            from.x -= 1.1*max_extent;
            self.draw_grid_value(ctx, design, y_label, from, false, 0.0, 0.0);
        }
        self.draw_scale_names(ctx, design);
        ctx.restore();
    }

    fn draw_scale_names(&self, ctx : &Context, design : &PlotDesign) {
        let pos_x = Coord2D::new(
            self.mapper.w as f64 * 0.5,
            self.mapper.h as f64 * 0.975
        );
        // export POS_X=0.1
        let pos_y = Coord2D::new(
            self.mapper.w as f64 * 0.025,
            self.mapper.h as f64 * 0.5
        );
        text::draw_label(
            &design.font.sf,
            ctx,
            &self.x.label[..],
            pos_x,
            false,
            (true, true),
            None,
            None
        );
        text::draw_label(
            &design.font.sf,
            ctx,
            &self.y.label[..],
            pos_y,
            true,
            (true, true),
            None,
            None
        );
    }

    /*fn update_mapping_name(name : &str) {
        // Verify if mapping name is not x|y|design|
    }*/

    /// For each mapping, return a tuple with (name, type, properties).
    pub fn mapping_info(&self) -> Vec<(String, String, HashMap<String,String>)> {
        let mut info = Vec::new();
        for (i, m) in self.mappings.iter().enumerate() {
            info.push((i.to_string(), m.mapping_type(), m.properties()))
        }
        //println!("{:?}", info);
        info
    }

    pub fn mapping_column_names(&self, id : &str) -> Vec<(String, String)> {
        let mut names = Vec::new();
        if let Some(m) = self.mappings.get(id.parse::<usize>().unwrap()) {
            names.extend(m.get_ordered_col_names());
        }
        names
    }

    pub fn scale_info(&self, scale : &str) -> HashMap<String, String> {
        match scale {
            "x" => self.x.description(),
            "y" => self.y.description(),
            _ => HashMap::new()
        }
    }

    pub fn update_mapping_columns(
        &mut self,
        id : &str,
        columns : Vec<String>
    ) -> Result<(), Box<dyn Error>> {
        if let Some(mapping) = self.mappings.get_mut(id.parse::<usize>().unwrap()) {
            if let Err(e) = mapping.set_col_names(columns) {
                println!("{}", e);
            }
        } else {
            println!("Mapping not found when updating column name");
        }
        if let Err(e) = self.reload_layout_node() {
            println!("{}", e);
        }
        Ok(())
    }

    pub fn update_source(
        &mut self,
        id : &str,
        source : String
    ) -> Result<(), Box<dyn Error>> {
        if let Some(mapping) = self.mappings.get_mut(id.parse::<usize>().unwrap()) {
            mapping.set_source(source);
        } else {
            println!("Mapping not found when updating column name");
        }
        Ok(())
    }

    pub fn view_sources(&self) -> Vec<String> {
        self.mappings.iter().map(|mapping| mapping.get_source() ).collect()
    }

    /*pub fn update_mapping_column(
        &mut self,
        id : &str,
        column : &str,
        name : &str
    ) {
        if let Some(mapping) = self.mappings.get_mut(id.parse::<usize>().unwrap()) {
            mapping.set_col_name(column, name);
        } else {
            println!("Mapping not found when updating column name");
        }
        if let Err(e) = self.reload_layout_data() {
            println!("{}", e);
        }
    }*/

    /*pub fn get_mapping_column(
        &self,
        id : &str,
        column : &str
    ) -> Option<String> {
        if let Some(mapping) = self.mappings.get(id.parse::<usize>().unwrap()) {
            let col_name = mapping.get_col_name(column);
            if col_name != "None" {
                Some(col_name)
            } else {
                None
            }
        } else {
            println!("Mapping not found when getting column name");
            None
        }
    }*/

}

//#[repr(C)]

pub mod utils {

    use super::Node;
    use super::HashMap;
    use super::Document;
    use super::Error;

    /// Return all children of node that satisfy the
    /// informed xpath.
    pub fn children_as_hash(
        node : &Node,
        xpath : &str
    ) -> HashMap<String, String> {
        let mut prop_hash = HashMap::new();
        if let Ok(props) = node.findnodes(xpath) {
            if props.len() == 0 {
                panic!("No children found for node {:?} at path {}", node, xpath);
            }
            for prop in props.iter() {
                // println!("Property = {:?}", prop);
                let name = prop.get_attribute("name")
                    .expect(&format!("No name attribute found for property {:?}", prop));
                let value = prop.get_content();
                prop_hash.insert(name, value);
            }
        } else {
            panic!("Failed to retrieve children of {:?} at path {}", node, xpath);
        }
        prop_hash
    }

    pub fn populate_node_with_hash(
        doc : &Document,
        node : &mut Node,
        hash : HashMap<String, String>
    ) -> Result<(), Box<dyn Error>> {
        for (k, v) in hash {
            let mut property = Node::new(
                "property", Option::None, doc).unwrap();
            property.set_attribute("name", &k[..])?;
            property.set_content(&v[..])?;
            node.add_child(&mut property)?;
        }
        Ok(())
    }

    pub fn edit_node_with_hash(
        doc : &Document,
        props : &HashMap<String, String>,
        node : &mut Node
    ) {
        let mut keys : Vec<String> = props.iter().map(|(k, v)| k.clone() ).collect();
        // println!("Keys: {:?}", keys);
        let mut n_changed = 0;
        // println!("Child nodes: {:?}", node.get_child_nodes().iter().map(|node| format!("{} {:?}", node.get_name(), node.get_property("name"))).collect::<Vec<_>>() );
        for mut child in node.get_child_nodes().iter_mut() {
            if &child.get_name()[..] == "property" {
                if let Some(name) = child.get_attribute("name") {
                    // if keys.iter().find(|k| &k[..] == &name[..] ).is_some() {
                    child.set_content(&props[&name]).unwrap();
                    n_changed += 1;
                    //} else {
                    //    println!("No property named {}", name);
                    //}
                } else {
                    println!("Node does not have name property");
                }
            }
        }
        let n_required = props.iter().count();
        if n_changed != n_required {
            println!("Changed only {} nodes (of {} required)", n_changed, n_required);
        }
    }

}

//impl IsA<gtk::DrawingArea> for PlotView {
//}

//Draw
/*impl ObjectImpl for PlotView {

    glib_object_impl!();

    //fn get_type_data(&self) -> NonNull<TypeData> {
    //}

    //glib_wrapper! {
    //}
}*/
//impl AsRef
//unsafe impl IsA<gtk::DrawingArea> for PlotView {
//}
/*impl ObjectSubclass for PlotView {
    const NAME: &'static str = "PlotView";
    type ParentType = gtk::DrawingArea;
    /* Glib classes are global runtime structs that are created
    when the first object of a given class is instantiated,
    and are destroyed when the last object of a given class
    is destroyed. (There is only a single instance of each
    class at any given time). The alias "Class" automatically
    implements a boilerplate struct to hold this class. */
    type Class = subclass::simple::ClassStruct<Self>;
    /* The instante is a global runtime struct (also one for
    each registered object) that describes things like
    memory object layout. Also automatically created. */
    type Instance = subclass::simple::InstanceStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        let plot_area = Plot::new(String::from("assets/layout.xml"));
        PlotView{plot_area}
    }
}*/
// glib::Object::new(T::get_type(), &[])
// get_type() registers type
// glib_wrapper!

// Used for overriding virtual methods - Must map to
// Impl trait
//unsafe impl IsSubclassable<PlotView>
//for gtk::auto::drawing_area::DrawingAreaClass {

//}

//subclass::types::register_type();

/*impl ObjectSubclass for PlotView {
    const NAME: &'static str = "PlotView";

    type ParentType = gtk::DrawingArea;

    type Instance = PlotView;
    type Class = PlotViewClass;

    glib_object_subclass!();

    fn class_init(klass: &mut PlotView) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        PlotView::new();
    }
}*/
/*fn add_signal(
    &mut self,
    name: &str,
    flags: SignalFlags,
    arg_types: &[Type],
    ret_type: Type
)*/
//unsafe extern "C" fn
