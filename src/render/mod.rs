/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use cairo::Context;
use std::default::Default;
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::io::{ErrorKind, Write};
use mappings::*;
use std::fmt::Display;
use std::any::Any;
use std::error;
use std::{fmt, fs::File};
use cairo::{SvgSurface, PsSurface, ImageSurface, Format};
use std::path::Path;
use std::cmp::Ordering;
use std::mem;
use std::str::FromStr;
use std::process::Command;
use tempfile;
use std::fs;
use crate::model::Adjustment;

pub mod mappings;

pub mod context_mapper;

use context_mapper::{ContextMapper, Coord2D};

pub mod plot_design;

use plot_design::*;

pub mod scale;

pub use scale::*;

pub use mappings::bar::*;

pub use mappings::scatter::*;

pub use mappings::line::*;

pub use mappings::text::*;

pub use mappings::area::*;

pub use mappings::interval::*;

mod text;

use text::FontData;

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
            "Unique" | "unique" => Ok(Self::Unique),
            "Four" | "four" => Ok(Self::Four),
            "Horizontal" | "horizontal" => Ok(Self::Horizontal),
            "Vertical" | "vertical" => Ok(Self::Vertical),
            "ThreeLeft" | "threeleft"=> Ok(Self::ThreeLeft),
            "ThreeTop" | "threetop" => Ok(Self::ThreeTop),
            "ThreeRight" | "threeright" => Ok(Self::ThreeRight),
            "ThreeBottom" | "threebottom" => Ok(Self::ThreeBottom),
            _ => Err(())
        }
    }

}

fn n_plots_for_split(split : &GroupSplit) -> usize {
    match split {
        GroupSplit::Unique => 1,
        GroupSplit::Vertical | GroupSplit::Horizontal => 2,
        GroupSplit::ThreeLeft | GroupSplit::ThreeTop | GroupSplit::ThreeRight | GroupSplit::ThreeBottom => 3,
        GroupSplit::Four => 4,
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

pub enum IntervalProperty {
    Color(String),
    Width(f64),
    Dash(i32),
    Center(Vec<f64>),
    Lower(Vec<f64>),
    Upper(Vec<f64>),
    Limit(f64),
    Vertical(bool)
}

/// Must come with plot position and mapping position within plot
pub enum MappingProperty {
    Line(LineProperty),
    Scatter(ScatterProperty),
    Text(TextProperty),
    Interval(IntervalProperty)
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

/// A Panel is a set of 1-4 plots with a given layout. Regions with arbitrary
/// number of plots can be built by splitting it according to some aspect
/// ratio and drawing multiple panels to it. To draw a 3x3 grid, for example,
/// use a 2x2 panel at the top-left, a 1x2 at the bottom, a 2x1 at the right
/// and a 1x1 at the bottom-right, such that the aspect ratios of the regions
/// are such that the different layouts do not matter for the final output.
/// TODO perhaps rename this 1-4 unit to "composition" and rename the set
/// of compositions as "Panel".
#[derive(Clone)]
pub struct Panel {

    design : PlotDesign,

    plots : Vec<Plot>,

    split : GroupSplit,

    h_ratio : f64,

    v_ratio : f64,

    dimensions : (usize, usize),

}

unsafe impl Send for Panel { }

unsafe impl Sync for Panel { }

unsafe impl Send for Plot { }

unsafe impl Sync for Plot { }

impl Default for Panel {

    fn default() -> Self {
        Self {
            design : Default::default(),
            plots : vec![Plot::default()],
            split : GroupSplit::Unique,
            h_ratio : 0.5,
            v_ratio : 0.5,
            dimensions : (800, 600)
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

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct FileError(String);

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical
}

fn update_dims_from_env(dims : &mut (usize, usize)) {
    if let Ok(var) = std::env::var("PLOT_DEFAULT_WIDTH") {
        dims.0 = var.parse().unwrap();
    }
    if let Ok(var) = std::env::var("PLOT_DEFAULT_HEIGHT") {
        dims.1 = var.parse().unwrap();
    }
}

impl Panel {

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn dimensions(mut self, w : u32, h : u32) -> Self {
        self.dimensions.0 = w as usize;
        self.dimensions.1 = h as usize;
        self.adjust_scales();
        self
    }

    pub fn single(p1 : Plot) -> Self {
        let mut panel = Self::default();
        update_dims_from_env(&mut panel.dimensions);
        panel.dimensions = (p1.mapper.w as usize, p1.mapper.h as usize);
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
        update_dims_from_env(&mut group.dimensions);
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

    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from_mapping(mapping : crate::model::Mapping) -> Result<Self, String> {
        let mut plot = crate::model::Plot::default();
        // TODO adjust scale for single-data mappings (bar)
        plot.x = crate::model::Scale::new_adjusted(mapping.map.x.as_ref().ok_or("Missing x".to_owned())?)
            .map_err(|e| format!("{}",e) )?;
        plot.y = crate::model::Scale::new_adjusted(mapping.map.y.as_ref().ok_or("Missing y".to_owned())?)
            .map_err(|e| format!("{}",e) )?;
        plot.mappings = vec![mapping];
        Self::new_from_single(plot)
    }

    pub fn new_from_single(plot : crate::model::Plot) -> Result<Self, String> {
        
        plot.validate().map_err(|e| format!("{}", e) )?;
        
        let design_json = plot.design.clone().unwrap_or_default();
        let layout_json = plot.layout.clone().unwrap_or_default();
        let design = PlotDesign::new_from_json(design_json)
            .map_err(|e| format!("Invalid design: {}", e))?;
        let area = Plot::new_from_model(plot)
            .map_err(|e| format!("Invalid plot: {}", e) )?;
        Ok(Self {
            design,
            plots : vec![area],
            split : GroupSplit::Unique,
            h_ratio : layout_json.hratio,
            v_ratio : layout_json.vratio,
            dimensions : (layout_json.width as usize, layout_json.height as usize),
        })
    }

    pub fn new_from_model(mut panel_def : crate::model::Panel) -> Result<Self, String> {
    
        panel_def.validate().map_err(|e| format!("{}", e) )?;
        
        let mut panel : Panel = Default::default();
        panel.plots.clear();

        if panel_def.plots.len() == 1 {
            panel.split = GroupSplit::Unique;
        } else {
            if panel_def.plots.len() == 2 {
                panel.split = GroupSplit::Horizontal;
            } else {
                if panel_def.plots.len() == 3 {
                    panel.split = GroupSplit::ThreeTop;
                } else {
                    if panel_def.plots.len() == 4 {
                        panel.split = GroupSplit::Four;
                    } else {
                        return Err(format!("Invalid number of plots informed"));
                    }
                }
            }
        }

        // Always ignore the layout/design of individual plot elements
        // when they are inside a panel definition. The individual layout/design
        // for separate plots only apply when they are a single element with an
        // implicit panel definition.
        for mut plot_def in panel_def.plots.drain(..) {

            // Just overwrite them if set at the panel level.
            if plot_def.design.is_some() {
                plot_def.design = None;

            }
            if plot_def.layout.is_some() {
                plot_def.layout = None;
            }

            let plot = Plot::new_from_model(plot_def)
                .map_err(|e| format!("Invalid plot: {}", e) )?;
            panel.plots.push(plot);
        }

        if let Some(design) = panel_def.design {
            panel.design = PlotDesign::new_from_json(design)
                .map_err(|e| format!("{}", e) )?;
        }

        if let Some(layout) = panel_def.layout {
            panel.dimensions = (layout.width as usize, layout.height as usize);
            panel.h_ratio = layout.hratio;
            panel.v_ratio = layout.vratio;

            if let Some(split) = &layout.split {
                let split = GroupSplit::from_str(split)
                    .map_err(|_| format!("Invalid split: {}", split))?;
                if n_plots_for_split(&split) == panel.plots.len() {
                    panel.split = split;
                } else {
                    // Do not set user-defined split property in case it was miss-specified, use
                    // the default for the given number of plots informed.
                    let n_plots = panel.plots.len();
                    panel.split = match n_plots  {
                        1 => GroupSplit::Unique,
                        2 => GroupSplit::Horizontal,
                        3 => GroupSplit::ThreeTop,
                        4 => GroupSplit::Four,
                        _ => return Err(String::from("More than four plots found"))
                    };
                }
            }
        }
        assert!(panel.plots.len() == n_plots_for_split(&panel.split), "N plots = {}; split = {:?}", panel.plots.len(), panel.split);
        Ok(panel)
    }

    pub fn new_from_json(json : &str) -> Result<Self, String> {
        let v : serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("{}", e) )?;
        let can_be_panel = v.get("plots").is_some();
        let can_be_plot = v.get("mappings").is_some();
        if can_be_panel && !can_be_plot {
            let panel_def : crate::model::Panel = serde_json::from_value(v)
                .map_err(|e| format!("{}", e) )?;
            Self::new_from_model(panel_def)
        } else if !can_be_panel && can_be_plot {
            let plot_def : crate::model::Plot = serde_json::from_value(v)
                .map_err(|e| format!("{}", e) )?;
            Self::new_from_single(plot_def)
        } else {
            let m : Result<crate::model::Mapping, _> = serde_json::from_value(v);
            if let Ok(m) = m {
                Self::new_from_mapping(m)
            } else {
                Err("Invalid top-level fields (expected one of 'mappings' or 'plots' array)".to_string())
            }
        }
    }

    pub fn adjust_scales(&mut self) {
        self.plots.iter_mut().for_each(|pl| pl.adjust_scales() );
    }

    pub fn clear_all_data(&mut self) {
        for area in self.plots.iter_mut() {
            area.clear_all_data();
        }
    }

    pub fn png(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let surf = ImageSurface::create(
            Format::ARgb32,
            self.dimensions.0 as i32,
            self.dimensions.1 as i32,
        )?;
        let ctx = Context::new(&surf).unwrap();
        self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32)?;
        let mut buf = Vec::new();
        surf.write_to_png(&mut buf)?;
        Ok(buf)
    }

    pub fn html_img_tag(&mut self) -> Result<String, Box<dyn Error>> {
        let png = self.png()?;
        Ok(format!("<img src='data:image/png;base64,{}' />", base64::encode(png)))
    }

    pub fn svg(&mut self) -> Result<String, Box<dyn Error>> {
        let svg_buf : Vec<u8> = Vec::new();
        let surf = SvgSurface::for_stream(
            self.dimensions.0 as f64,
            self.dimensions.1 as f64,
            svg_buf
        ).map_err(|e| format!("Error creating SVG surface: {}", e) )?;

        let ctx = Context::new(&surf).unwrap();
        self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32)?;

        let stream = surf.finish_output_stream().unwrap();

        surf.flush();

        Ok(String::from_utf8(stream.downcast_ref::<Vec<u8>>().unwrap().clone())?)
    }

    pub fn show_with_eog(&mut self) -> Result<(), Box<dyn Error>> {
        self.show_with_app("eog")
    }

    /// Shows plot by saving it at a tempfile and opening with the
    /// informed application, which is assumed to receive the tempfile
    /// path as first argument.
    pub fn show_with_app(&mut self, app : &str) -> Result<(), Box<dyn Error>> {
        let mut tf = tempfile::NamedTempFile::new()?;
        let png = self.png()?;
        tf.write_all(&png)?;
        let path = tf.path();
        let new_path = format!("{}.png", path.to_str().unwrap());
        fs::rename(path, new_path.clone()).unwrap();
        Command::new(app)
            .args(&[&new_path])
            .output()?;
        Ok(())
    }

    pub fn draw_to_file(&mut self, path : &str) -> Result<(), Box<dyn Error>> {
        // TODO Error creating SVG surface: "error while writing to output stream

        let path = Path::new(path);
        if !path.parent().map(|par| par.exists() ).unwrap_or(false) {
            Err(FileError(format!("Parent directory for image path {} does not exists", path.to_str().unwrap())))?;
        }

        match path.extension().and_then(|e| e.to_str() ) {
            Some("svg") => {
                let surf = SvgSurface::new(
                    self.dimensions.0 as f64,
                    self.dimensions.1 as f64,
                    Some(path)
                ).map_err(|e| FileError(format!("Error creating SVG surface: {}", e) ))?;
                let ctx = Context::new(&surf).unwrap();
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32)?;
            },
            Some("png") => {
                let surf = ImageSurface::create(
                    Format::ARgb32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                ).map_err(|e| FileError(format!("Error creating PNG image surface: {}", e) ))?;
                let ctx = Context::new(&surf).unwrap();
                // ctx.scale(3.0, 3.0);
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32)?;
                let mut f = File::create(path).map_err(|e| FileError(format!("Unable to open PNG file:{}", e)))?;
                surf.write_to_png(&mut f)
                    .map_err(|e| format!("Error writing content to png: {}", e) )?;
            },
            Some("eps") => {
                let surf = PsSurface::new(
                    self.dimensions.0 as f64,
                    self.dimensions.1 as f64,
                    path
                ).map_err(|e| FileError(format!("Error creating Postscript surface: {}", e) ))?;
                surf.set_eps(true);
                let ctx = Context::new(&surf).unwrap();
                self.draw_to_context(&ctx, 0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32)?;
            },
            Some(other) => {
                Err(FileError(format!("Invalid image export extension: {}", other)))?;
            },
            None => {
                Err(FileError(format!("No valid extension informed for image export file")))?;
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
    ) -> Result<(), Box<dyn Error>> {
        let top_left = (0.05, 0.05);
        let top_right = (w as f64 * self.h_ratio, 0.05);
        let bottom_left = (0.05, h as f64 * self.v_ratio);
        let bottom_right = (w as f64 * self.h_ratio, h as f64 * self.v_ratio);

        // The plot context mapper is re-set here, so plot must be mutably-borrowed
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
            ctx.save()?;
            ctx.translate(origin.0, origin.1);
            plot.draw_plot(&ctx, &self.design, size.0, size.1)?;
            ctx.restore()?;
        }
        Ok(())
    }

    pub fn update_mapping(&mut self, ix : usize, id : &str, data : &Vec<Vec<f64>>) -> Result<(), Box<dyn Error>> {
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

    pub fn ordered_col_names(&self, ix : usize, id : &str) -> Vec<(String, String)> {
        self.plots[ix].mapping_column_names(id)
    }

    pub fn scale_info(&self, ix : usize, scale : &str) -> HashMap<String, String> {
        self.plots[ix].scale_info(scale)
    }

    pub fn design_info(&self) -> HashMap<String, String> {
        self.design.description()
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

    pub fn view_dimensions(&self) -> (u32, u32) {
        (self.dimensions.0 as u32, self.dimensions.1 as u32)
    }

}

#[derive(Clone, Debug)]
pub struct Plot {
    mappings : Vec<Box<dyn Mapping>>,
    mapper : ContextMapper,
    x : Scale,
    y : Scale,
}

impl Default for Plot {
    fn default() -> Self {
        let mappings = Vec::new();
        let mapper : ContextMapper = Default::default();
        let x : Scale = Default::default();
        let y : Scale = Default::default();
        Plot{ mappings, mapper, x, y, }
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

impl Plot {

    // This only makes sense if we have a single plot that will be promptly
    // wrappen into a panel for drawing only. All dimensions are overwrritten
    // when we wrap multiple plots into a planel according to the plot split logic
    // (a step done at the drawing stage).
    pub fn dimensions(mut self, w : u32, h : u32) -> Self {
        self.mapper.update_dimensions(w as i32, h as i32);
        self.adjust_scales();
        self
    }

    pub fn wrap(&self) -> Panel {
        Panel::single(self.clone())
    }

    pub fn svg(&self) -> String {
        self.wrap().svg().unwrap()
    }

    pub fn draw_to_file(&self, path : &str) -> Result<(), Box<dyn Error>> {
        self.wrap().draw_to_file(path)
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

        if let Some(((new_xmin, mut new_xmax), (new_ymin, mut new_ymax))) = self.max_data_limits() {

            let min_x_spacing = self.x.n_intervals as f64 * std::f64::EPSILON;
            let min_y_spacing = self.y.n_intervals as f64 * std::f64::EPSILON;

            // Plots with extension zero are not valid - We hard-set the smallest possible difference,
            // or else the scale drawing will be messed up. This might happen if the user provide a single
            // data point for the mapping, in which case xmax == xmin. Each grid point must be distant by at least EPS.
            if (new_xmax - new_xmin).abs() < min_x_spacing {
                new_xmax = new_xmin + min_x_spacing;
            }
            if (new_ymax - new_ymin).abs() < min_y_spacing {
                new_ymax = new_ymin + min_y_spacing;
            }

            let (x_adj, y_adj) = (self.x.adj, self.y.adj);
            scale::adjust_segment(&mut self.x, x_adj, new_xmin, new_xmax);
            scale::adjust_segment(&mut self.y, y_adj, new_ymin, new_ymax);
            self.mapper.update_data_extensions(self.x.from, self.x.to, self.y.from, self.y.to);

        } else {
            // println!("Could not retrieve data limits");
        }
    }

    pub fn new_from_json(json : &str) -> Result<Plot, Box<dyn Error>> {
        let plot : crate::model::Plot = serde_json::from_str(&json)?;
        Self::new_from_model(plot)
    }

    pub fn new_from_model(mut rep : crate::model::Plot) -> Result<Plot, Box<dyn Error>> {

        let mut mappings = Vec::new();

        for mapping in rep.mappings.iter_mut() {
            mappings.push(mappings::new_from_json(mem::take(mapping))?);
        }

        let x = Scale::new_from_json(rep.x.clone())?;
        let y = Scale::new_from_json(rep.y.clone())?;

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
        };
        area.adjust_scales();

        // We do not load any design definitions here, but rather at Panel::new(),
        // since the design might be defined at Panel-level.
        Ok(area)
    }

    pub fn new() -> Self {
        let mut pl : Plot = Default::default();
        if let Ok(var) = std::env::var("PLOT_DEFAULT_WIDTH") {
            pl.mapper.w = var.parse().unwrap();
        }
        if let Ok(var) = std::env::var("PLOT_DEFAULT_HEIGHT") {
            pl.mapper.h = var.parse().unwrap();
        }
        pl
    }

    fn draw_plot(&mut self, ctx: &Context, design : &PlotDesign, w : i32, h : i32) -> Result<(), Box<dyn Error>> {
        self.mapper.update_dimensions(w, h);
        self.draw_background(ctx, design)?;
        self.draw_grid(ctx, design)?;
        for mapping in self.mappings.iter() {
            mapping.draw(&self.mapper, &ctx)?;
        }
        Ok(())
    }

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

    fn accomodate_dimension(
        &mut self,
        data : &[f64],
        old_min : f64,
        old_max : f64,
        _dim_name : &str
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
            /*let ans = self.update_layout(
                &format!("object[@name='{}']/property[@name='from']", dim_name)[..],
                &new_min.to_string()
            );
            if let Err(e) = ans {
                println!("{}", e);
            }*/
        }
        if new_max > old_max {
            /*let ans = self.update_layout(
                &format!("object[@name='{}']/property[@name='to']", dim_name)[..],
                &new_max.to_string()
            );
            if let Err(e) = ans {
                println!("{}", e);
            }*/
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
                },
                _ => {
                    println!("Invalid mapping type");
                    return;
                }
            }
            m.update_data(empty_data);
        }
    }

    fn draw_background(&self, ctx : &Context, design : &PlotDesign) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
        ctx.set_line_width(0.0);
        ctx.set_source_rgb(
            design.bg_color.red().into(),
            design.bg_color.green().into(),
            design.bg_color.blue().into()
        );
        ctx.rectangle(
            context_mapper::REL_X_OFFSET*(self.mapper.w as f64), 
            context_mapper::REL_Y_OFFSET*(self.mapper.h as f64),
            context_mapper::REL_WIDTH*(self.mapper.w as f64), 
            context_mapper::REL_HEIGHT*(self.mapper.h as f64)
        );
        ctx.fill()?;
        ctx.restore()?;
        Ok(())
    }

    fn draw_grid_line(
        &self,
        ctx : &Context,
        design : &PlotDesign,
        from : Coord2D,
        to : Coord2D
    ) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
        ctx.set_source_rgb(
            design.grid_color.red().into(),
            design.grid_color.green().into(),
            design.grid_color.blue().into()
        );
        ctx.move_to(from.x, from.y);
        ctx.line_to(to.x, to.y);
        ctx.stroke()?;
        ctx.restore()?;
        Ok(())
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
    ) -> Result<(), Box<dyn Error>> {
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
        )?;
        Ok(())
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
            .map(|l| design.font.sf.text_extents(&l[..]).x_advance())
            .fold(0.0, |m, f| f64::max(m,f))
    }

    fn draw_grid(&self, ctx : &Context, design : &PlotDesign) -> Result<(), Box<dyn Error>> {
        ctx.save()?;
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
            
            self.draw_grid_line(ctx, design, from, to)?;
            self.draw_grid_value(ctx, design, x_label, from, true, 0.0, 1.5)?;
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
            self.draw_grid_line(ctx, design, from, to)?;
            //let mut y_label_coord = match self.mapper.yinv {
            //    true => to,
            //    false => from
            //};
            from.x -= 1.1*max_extent;
            self.draw_grid_value(ctx, design, y_label, from, false, 0.0, 0.0)?;
        }
        self.draw_scale_names(ctx, design)?;
        ctx.restore()?;
        Ok(())
    }

    fn draw_scale_names(&self, ctx : &Context, design : &PlotDesign) -> Result<(), Box<dyn Error>> {
        let pos_x = Coord2D::new(
            self.mapper.w as f64 * 0.5,
            self.mapper.h as f64 * 0.975
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
        )?;
        
        // export POS_X=0.1
        let pos_y = Coord2D::new(
            self.mapper.w as f64 * 0.005, // self.mapper.w as f64 * 0.025,
            self.mapper.h as f64 * 0.5
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
        )?;
        Ok(())
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

}

