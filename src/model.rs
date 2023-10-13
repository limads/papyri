/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use serde::{Serialize, Deserialize};
use std::default::Default;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;
use std::cmp::{PartialEq, Eq};
use std::error::Error;

/*
// Drawing primitives shared by multiple mappings.
pub mod primitives {

}

// Conventional visualizations that can be built from one or more sets of mappings,
possibly requiring predictable data manipulations.
pub mod recipes {

    histogram
    boxplot
    violin
    density
    qqplot
    matrix(pl, a, b) Cartesian product of factors a, b for arbitrary plots pl.
    tree (dendrogram-like visualizations)
    
}

Color or shape mapping labels.
pub mod labels {

}

*/

/*
The minimal JSON satisfying the plot spec is:
{
    "x" : {},
    "y" : {},
    "mappings" : {}
}

The minimal JSON satisfying the panel spec is:
{
    "design" : {},
    "layout" : {},
    "plots" : {}
}
*/

/* It is perfectly possible that the structures model::Plot, model::Panel and model::Mapping at
this module hold invalid states. Cairo never sees those structures directly. The actual validation
is done when the render::Plot, render:Panel and the concrete mappings are built from the loosely-typed
structures here. Those structures only represent a valid de-serialization step from a plot definition
handed by the user. */

/// Carries the logic of how scale adjustment to the mapping should be done.
/// Tight means the two scales will extend just enough to show the data.
/// Round means the scales will extend to show the data, and a little
/// more so that it ends at a nearby round number at the scale of 5 or 10.
/// Off means adjustment is not applied, and the minimum and maximum
/// values supplied by the user will be used instead.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Adjustment {
    Tight,

    Round,

    Off
}

impl FromStr for Adjustment {

    type Err = ();

    fn from_str(s : &str) -> Result<Self, ()> {
        match s {
            "tight" => Ok(Self::Tight),
            "round" => Ok(Self::Round),
            "off" => Ok(Self::Off),
            _ => Err(())
        }
    }

}

impl Default for Adjustment {

    fn default() -> Self {
        Self::Off
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn like_any(&self, mappings : &[MappingType]) -> bool {
        mappings.iter().any(|m| *m == *self )
    }
    
    /*/// Returns a default property map for this mapping type. This is the major
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
                hash.insert(String::from("center"), String::from("false"));
                hash.insert(String::from("horizontal"), String::from("false"));
                hash.insert(String::from("width"), String::from("None"));
                hash.insert(String::from("height"), String::from("None"));
                hash.insert(String::from("width"), String::from("100"));
                hash.insert(String::from("origin_x"), String::from("0"));
                hash.insert(String::from("origin_y"), String::from("0"));
                hash.insert(String::from("spacing"), String::from("1"));
            },
            MappingType::Area => {
                hash.insert(String::from("ymax"), String::from("None"));
                hash.insert(String::from("opacity"), String::from("1.0"));
            },
            /*MappingType::Surface => {
                hash.insert(String::from("z"), String::from("None"));
                hash.insert(String::from("final_color"), String::from("#ffffff"));
                hash.insert(String::from("z_min"), String::from("0.0"));
                hash.insert(String::from("z_max"), String::from("1.0"));
                hash.insert(String::from("opacity"), String::from("1.0"));
            },*/
            MappingType::Text => {
                hash.insert(String::from("font"), String::from("Monospace Regular 12"));
                hash.insert(String::from("text"), String::from("None"));
            },
            MappingType::Interval => {
                unimplemented!()
            }
        }
        hash
    }*/
}

/*
The design and layout are always applied to the panel. When single plots
carry a layout and design, they refer to the design and layout of the implicit
1-element panel
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Design {
    pub bgcolor : String,
    pub fgcolor : String,
    pub width : i32,
    pub font : String
}

#[derive(Debug, Default)]
pub struct DesignBuilder(Design);

impl DesignBuilder {

    pub fn build(self) -> Design {
        self.0
    }

    pub fn bgcolor(mut self, s : &str) -> Self {
        self.0.bgcolor = s.to_string();
        self
    }

    pub fn fgcolor(mut self, s : &str) -> Self {
        self.0.fgcolor = s.to_string();
        self
    }

    pub fn font(mut self, s : &str) -> Self {
        self.0.font = s.to_string();
        self
    }

    pub fn width(mut self, width : i32) -> Self {
        self.0.width = width;
        self
    }

}

/// Represents a design definition error propagated to the user.
#[derive(Debug, thiserror::Error)]
pub enum DesignError {

    #[error("Invalid grid width")]
    InvalidGridWidth,

    #[error("Invalid grid color")]
    InvalidGridColor,

    #[error("Invalid background color")]
    InvalidBackgroundColor
}

impl Design {

    pub fn validate(&self) -> Result<(), DesignError> {
        if self.width < 0 || self.width > 50 {
            Err(DesignError::InvalidGridWidth)?;
        }
        if !crate::model::validate_color(&self.fgcolor) {
            Err(DesignError::InvalidGridColor)?;
        }
        if !crate::model::validate_color(&self.bgcolor) {
            Err(DesignError::InvalidBackgroundColor)?;
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> DesignBuilder {
        DesignBuilder(Design::default())
    }

}

impl Default for Design {
    fn default() -> Self {
        Self {
            bgcolor : String::from("#ffffff"),
            fgcolor : String::from("#d3d7cf"),
            width : 1,
            font : String::from("Monospace Regular 22")
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Layout { ratio : f64, stacked : bool }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
    pub width : i32,
    pub height : i32,
    pub hratio : f64,
    pub vratio : f64,
    pub split : Option<String>
}

#[derive(Debug, thiserror::Error)]
pub enum LayoutError {

    #[error("'width' should be strictly positive")]
    Width,
    
    #[error("'height' should be strictly positive")]
    Height,
    
    #[error("'hratio' should be in the interval 0.0 - 1.0")]
    HRatio,
    
    #[error("'vratio' should be in the interval 0.0 - 1.0")]
    VRatio,
    
    #[error("Invalid value for 'split'. Expected one of 'unique', 'horizontal', 'vertical', 'threetop', 'threebottom', 'threeleft', 'threeright', 'four'")]
    Split
    
}

const VALID_SPLITS : [&'static str; 8] = [
    "unique",
    "horizontal",
    "vertical",
    "threetop",
    "threebottom",
    "threeleft",
    "threeright",
    "four"
];

impl Layout {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> LayoutBuilder {
        LayoutBuilder(Layout::default())
    }
    
    pub fn validate(&self) -> Result<(), LayoutError> {
        if self.width < 0 {
            Err(LayoutError::Width)
        } else if self.height < 0 {
            Err(LayoutError::Height)
        } else if self.hratio < 0.0 || self.hratio > 1.0 {
            Err(LayoutError::HRatio)
        } else if self.vratio < 0.0 || self.vratio > 1.0 {
            Err(LayoutError::VRatio)
        } else if self.split.as_ref().map(|s| !(VALID_SPLITS.iter().any(|r| &s[..] == &r[..] )) ).unwrap_or(false) {
            Err(LayoutError::Split)
        } else {
            Ok(())
        }
    }

}

pub struct LayoutBuilder(Layout);

impl LayoutBuilder {

    pub fn build(self) -> Layout {
        self.0
    }

    pub fn width(mut self, width : i32) -> Self {
        self.0.width = width;
        self
    }

    pub fn height(mut self, height : i32) -> Self {
        self.0.height = height;
        self
    }

    pub fn hratio(mut self, ratio : f64) -> Self {
        self.0.hratio = ratio;
        self
    }

    pub fn vratio(mut self, ratio : f64) -> Self {
        self.0.vratio = ratio;
        self
    }

    pub fn split(mut self, split : &str) -> Self {
        self.0.split = Some(split.to_string());
        self
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width : 800,
            height : 600,
            hratio : 0.5,
            vratio : 0.5,
            split : None
        }
    }
}

/// Represents a mapping definition error propagated to the user.
#[derive(Debug, thiserror::Error)]
pub enum ScaleError {

    #[error("Inverted range")]
    InvertedRange,

    #[error("Number of steps")]
    StepNumber,

    #[error("Invalid offset")]
    InvalidOffset,

    #[error("Invalid adjustment")]
    InvalidAdjustment

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    pub label : String,
    pub from : f64,
    pub to : f64,
    pub precision : Option<i32>,
    pub intervals : Option<i32>,
    pub log : Option<bool>,
    pub invert : Option<bool>,
    pub offset : Option<i32>,
    pub adjust : Option<String>,
    pub guide : Option<bool>
}

impl Scale {

    pub fn new_adjusted(vs : &[f64]) -> Result<Self, ScaleError> {
        let mut scale = crate::model::Scale::default();
        scale.from = *vs.iter().min_by(|a, b| a.total_cmp(&b) )
            .ok_or(ScaleError::InvalidAdjustment)?;
        scale.to = *vs.iter().max_by(|a, b| a.total_cmp(&b) )
            .ok_or(ScaleError::InvalidAdjustment)?;
        Ok(scale)
    }

    pub fn validate(&self) -> Result<(), ScaleError> {
        if self.from > self.to {
            Err(ScaleError::InvertedRange)?;
        }
        if let Some(offset) = self.offset {
            if offset < 0 || offset > 100 {
                Err(ScaleError::InvalidOffset)?;
            }
        }
        if let Some(adj) = &self.adjust {
            if Adjustment::from_str(adj).is_err() {
                Err(ScaleError::InvalidAdjustment)?;
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        Scale::default()
    }

    pub fn builder() -> ScaleBuilder {
        ScaleBuilder(Scale::default())
    }

}

pub const DEFAULT_INVERT : bool = false;

pub const DEFAULT_LOG : bool = false;

pub const DEFAULT_OFFSET : i32 = 0;

pub const DEFAULT_INTERVALS : i32 = 5;

pub const DEFAULT_PRECISION : i32 = 2;

pub const DEFAULT_ADJUSTMENT : Adjustment = Adjustment::Off;

impl Default for Scale {
    fn default() -> Self {
        Self {
            label : String::new(),
            from : 0.0,
            to : 1.0,
            precision : Some(DEFAULT_PRECISION),
            intervals : Some(DEFAULT_INTERVALS),
            log : Some(DEFAULT_LOG),
            invert : Some(DEFAULT_INVERT),
            offset : Some(DEFAULT_OFFSET),
            adjust : Some(String::from("off")),
            guide : None
        }
    }
}

pub struct ScaleBuilder(Scale);

impl ScaleBuilder {

    pub fn build(self) -> Scale {
        self.0
    }

    pub fn guide(mut self, guide : bool) -> Self {
        self.0.guide = Some(guide);
        self
    }

    pub fn label(mut self, label : &str) -> Self {
        self.0.label = label.to_string();
        self
    }

    pub fn adjust(mut self, adjust : &str) -> Self {
        self.0.adjust = Some(adjust.to_string());
        self
    }

    pub fn precision(mut self, precision : i32) -> Self {
        self.0.precision = Some(precision);
        self
    }

    pub fn intervals(mut self, intervals : i32) -> Self {
        self.0.intervals = Some(intervals);
        self
    }

    pub fn offset(mut self, offset : i32) -> Self {
        self.0.offset = Some(offset);
        self
    }

    pub fn from(mut self, from : f64) -> Self {
        self.0.from = from;
        self
    }

    pub fn to(mut self, to : f64) -> Self {
        self.0.to = to;
        self
    }

    pub fn log(mut self, log : bool) -> Self {
        self.0.log = Some(log);
        self
    }

    pub fn invert(mut self, invert : bool) -> Self {
        self.0.invert = Some(invert);
        self
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Map {

    /* TODO apply custom parsing here. It is very common to
    get floating point values that are f64::NAN, which will
    be converted to Json's null. Inform the user when there
    are null values, instead of a generic error message. */

    //#[serde(deserialize_with = "deser_num_data")]
    pub x : Option<Vec<f64>>,

    //#[serde(deserialize_with = "deser_num_data")]
    pub y : Option<Vec<f64>>,

    //surface and area-specific
    //#[serde(deserialize_with = "deser_num_data")]
    pub z : Option<Vec<f64>>,

    // Text-specific
    //#[serde(deserialize_with = "deser_text_data")]
    pub text : Option<Vec<String>>
}

/*// This avoids conversion from NULL or null to serde_json::Value::Nil,
// (nulls can be represented as such on the textual data).
fn deser_text_data<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.to_string())
}

fn deser_num_data<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if let Ok(i) = s.parse::<i64>() {
        Ok(i as f64)
    } else {
        s.parse::<f64>().map_err(D::Error::custom)
    }
}*/

impl Map {

    // Returns true if the pair has the same set of valid fields.
    pub fn like(&self, other : &Self) -> bool {
        self.x.as_ref().xor(other.x.as_ref()).is_none() &&
        self.y.as_ref().xor(other.y.as_ref()).is_none() &&
        self.z.as_ref().xor(other.z.as_ref()).is_none() &&
        self.text.as_ref().xor(other.text.as_ref()).is_none()
    }
    
    pub fn empty_for_line() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn empty_for_scatter() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn empty_for_bar() -> Self {
        Self {
            x : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn empty_for_interval() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            z : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn empty_for_area() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            z : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn empty_for_surface() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            z : Some(Vec::new()),
            ..Default::default()
        }
    }

    pub fn empty_for_label() -> Self {
        Self {
            x : Some(Vec::new()),
            y : Some(Vec::new()),
            text : Some(Vec::new()),
            ..Default::default()
        }
    }
    
    pub fn description(&self) -> String {
        let mut s = String::from("(");
        if self.x.is_some() {
            s += "x,"
        }
        if self.y.is_some() {
            s += "y,"
        }
        if self.z.is_some() {
            s += "z,"
        }
        if self.text.is_some() {
            s += "t,"
        }
        s = s.trim_end_matches(",").to_string();
        s += ")";
        s
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub map : Map,
    pub width : f64,
    pub spacing : f64,
    pub color : String
}

pub struct LineBuilder(Line);

impl LineBuilder {

    pub fn build(self) -> Line {
        self.0
    }

    pub fn map(mut self, x : Vec<f64>, y : Vec<f64>) -> Self {
        self.0.map = Map { x : Some(x), y : Some(y), z : None, text : None };
        self
    }

    pub fn width(mut self, width : f64) -> Self {
        self.0.width = width;
        self
    }

    pub fn spacing(mut self, spacing : f64) -> Self {
        self.0.spacing = spacing;
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }
}

impl Line {

    pub fn new() -> Self {
        Line::default()
    }

    pub fn builder() -> LineBuilder {
        LineBuilder(Self::default())
    }

}

impl Default for Line {

    fn default() -> Self {
        Line {
            map : Map::empty_for_line(),
            width : 1.0,
            spacing : 1.0,
            color : String::from("#000000")
        }
    }

}

impl From<Line> for Mapping {

    fn from(line : Line) -> Self {
        let Line { map, width, spacing, color, .. } = line;
        Mapping { kind : String::from("line"), map : map, width : Some(width), spacing : Some(spacing), color : Some(color), ..Default::default() }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scatter {
    pub map : Map,
    pub radius : f64,
    pub color : String
}

pub struct ScatterBuilder(Scatter);

impl ScatterBuilder {

    pub fn build(self) -> Scatter {
        self.0
    }

    pub fn map(mut self, x : Vec<f64>, y : Vec<f64>) -> Self {
        self.0.map = Map { x : Some(x), y : Some(y), z : None, text : None };
        self
    }

    pub fn radius(mut self, radius : f64) -> Self {
        self.0.radius = radius;
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }
}

impl Scatter {

    pub fn new() -> Self {
        Scatter::default()
    }

    pub fn builder() -> ScatterBuilder {
        ScatterBuilder(Self::default())
    }

}

impl Default for Scatter {

    fn default() -> Self {
        Scatter {
            map : Map::empty_for_scatter(),
            radius : 10.0,
            color : String::from("#000000")
        }
    }

}

impl From<Scatter> for Mapping {

    fn from(scatter : Scatter) -> Self {
        let Scatter { map, radius, color, .. } = scatter;
        Mapping { kind : String::from("scatter"), map : map, radius : Some(radius), color : Some(color), ..Default::default() }
    }

}

// Interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interval {
    pub map : Map,
    pub width : f64,
    pub limits : f64,
    pub spacing : f64,
    pub vertical : bool,
    pub color : String
}

pub struct IntervalBuilder(Interval);

impl IntervalBuilder {

    // Line thickness
    pub fn width(mut self, width : f64) -> Self {
        self.0.width = width;
        self
    }

    // Spacing between elements if this interval is a dotted line.
    pub fn spacing(mut self, spacing : f64) -> Self {
        self.0.spacing = spacing;
        self
    }

    pub fn vertical(mut self, vertical : bool) -> Self {
        self.0.vertical = vertical;
        self
    }

    pub fn limits(mut self, limits : f64) -> Self {
        self.0.limits = limits;
        self
    }

    pub fn build(self) -> Interval {
        self.0
    }

    pub fn map(mut self, pos : Vec<f64>, min : Vec<f64>, max : Vec<f64>) -> Self {
        self.0.map = Map { x : Some(pos), y : Some(min), z : Some(max), text : None };
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }
}

impl Interval {

    pub fn new() -> Self {
        Interval::default()
    }

    pub fn builder() -> IntervalBuilder {
        IntervalBuilder(Self::default())
    }

}

impl Default for Interval {

    fn default() -> Self {
        Interval {
            map : Map::empty_for_interval(),
            color : String::from("#000000"),
            width : 1.0,
            spacing : 1.0,
            limits : 1.0,
            vertical : true
        }
    }

}

impl From<Interval> for Mapping {

    fn from(interval : Interval) -> Self {
        let Interval { map, color, width, vertical, limits, .. } = interval;
        Mapping {
            kind : String::from("interval"),
            map : map,
            color : Some(color),
            limits : Some(limits),
            vertical : Some(vertical),
            width : Some(width),
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub map : Map,
    pub color : String,
    pub font : String,
}

pub struct LabelBuilder(Label);

impl LabelBuilder {

    pub fn build(self) -> Label {
        self.0
    }

    pub fn map(mut self, x : Vec<f64>, y : Vec<f64>, text : Vec<String>) -> Self {
        self.0.map = Map { x : Some(x), y : Some(y), z : None, text : Some(text) };
        self
    }

    pub fn font(mut self, font : String) -> Self {
        self.0.font = font;
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }
}

impl Label {

    pub fn new() -> Label {
        Label::default()
    }

    pub fn builder() -> LabelBuilder {
        LabelBuilder(Self::default())
    }

}

impl Default for Label {

    fn default() -> Label {
        Label {
            map : Map::empty_for_label(),
            color : String::from("#000000"),
            font : String::from("Monospace Regular 22")
        }
    }

}

impl From<Label> for Mapping {

    fn from(label : Label) -> Self {
        let Label { map, color, font, .. } = label;
        Mapping { kind : String::from("text"), map : map, color : Some(color), font : Some(font), ..Default::default() }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub map : Map,
    pub color : String,
    pub width : f64,
    pub spacing : f64,
    pub origin : f64,
    pub center : bool,
    pub vertical : bool,
}

pub struct BarBuilder(Bar);

impl BarBuilder {

    pub fn build(self) -> Bar {
        self.0
    }

    pub fn map(mut self, x : Vec<f64>) -> Self {
        self.0.map = Map { x : Some(x), y : None, z : None, text : None };
        self
    }

    pub fn width(mut self, width : f64) -> Self {
        self.0.width = width;
        self
    }

    pub fn spacing(mut self, spacing : f64) -> Self {
        self.0.spacing = spacing;
        self
    }

    pub fn origin(mut self, origin : f64) -> Self {
        self.0.origin = origin;
        self
    }

    pub fn center(mut self, center : bool) -> Self {
        self.0.center = center;
        self
    }

    pub fn vertical(mut self, vertical : bool) -> Self {
        self.0.vertical = vertical;
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }
}

impl Bar {

    pub fn new() -> Bar {
        Bar::default()
    }

    pub fn builder() -> BarBuilder {
        BarBuilder(Self::default())
    }

}

impl Default for Bar {

    fn default() -> Bar {
        Bar {
            map : Map::empty_for_bar(),
            color : String::from("#000000"),
            width : 1.0,
            spacing : 1.0,
            origin : 0.,
            center : false,
            vertical : true,
        }
    }

}

impl From<Bar> for Mapping {

    fn from(bar : Bar) -> Self {
        let Bar { map, color, width, spacing, origin, center, vertical, .. } = bar;
        Mapping {
            kind : String::from("bar"),
            map : map,
            color : Some(color),
            origin : Some(origin),
            center :  Some(center),
            vertical : Some(vertical),
            width : Some(width),
            spacing : Some(spacing),
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
    pub map : Map,
    pub color : String,
    pub color_final : String,
    pub z_start : f64,
    pub z_end : f64
}

pub struct SurfaceBuilder(Surface);

impl SurfaceBuilder {

    pub fn build(self) -> Surface {
        self.0
    }

    pub fn map(mut self, x : Vec<f64>, y : Vec<f64>, z : Vec<f64>) -> Self {
        self.0.map = Map { x : Some(x), y : Some(y), z : Some(z), text : None };
        self
    }

    pub fn color(mut self, color : &str) -> Self {
        self.0.color = color.to_string();
        self
    }

    pub fn color_final(mut self, color : &str) -> Self {
        self.0.color_final = color.to_string();
        self
    }

    pub fn z_start(mut self, v : f64) -> Self {
        self.0.z_start = v;
        self
    }

    pub fn z_end(mut self, v : f64) -> Self {
        self.0.z_end = v;
        self
    }

}

impl Surface {

    pub fn new() -> Surface {
        Surface::default()
    }

    pub fn builder() -> SurfaceBuilder {
        SurfaceBuilder(Self::default())
    }

}

impl Default for Surface {

    fn default() -> Surface {
        Surface {
            map : Map::empty_for_bar(),
            color : String::from("#000000"),
            color_final : String::from("#000000"),
            z_start : 0.0,
            z_end : 1.0
        }
    }

}

impl From<Surface> for Mapping {

    fn from(surf : Surface) -> Self {
        let Surface { map, color, color_final, z_start, z_end } = surf;
        Mapping {
            kind : String::from("surface"),
            map : map,
            color : Some(color),
            color_final : Some(color_final),
            z_start : Some(z_start),
            z_end :  Some(z_end),
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    Kind,
    Color,
    Map,
    Width,
    Spacing,
    Vertical,
    Font,
    Radius,
    Limits,
    Center,
    Origin,
    ColorFinal,
    ZStart,
    ZEnd
}

impl Property {

    pub fn present(&self, m : &MappingType) -> bool {
        match self {
            Property::Kind | Property::Color | Property::Map => true,
            Property::Width | Property::Spacing => *m == MappingType::Line || *m == MappingType::Bar || *m == MappingType::Interval,
            Property::Vertical => *m == MappingType::Interval || *m == MappingType::Bar,
            Property::Font => *m == MappingType::Text,
            Property::Radius => *m == MappingType::Scatter,
            Property::Limits => *m == MappingType::Interval,
            Property::Center | Property::Origin => *m == MappingType::Bar,
            Property::ZStart | Property::ZEnd | Property::ColorFinal => *m == MappingType::Surface
        }
    }
    
    pub fn absent(&self, m : &MappingType) -> bool {
        !self.present(m)
    }
    
    pub fn name(&self) -> String {        
        match self {
            Self::Kind => format!("kind"),
            Self::Color => format!("color"),
            Self::Map => format!("map"),
            Self::Width => format!("width"),
            Self::Spacing => format!("spacing"),
            Self::Vertical => format!("vertical"),
            Self::Font => format!("font"),
            Self::Radius => format!("radius"),
            Self::Limits => format!("limits"),
            Self::Center => format!("center"),
            Self::Origin => format!("origin"),
            Self::ZStart => format!("zstart"),
            Self::ZEnd => format!("zend"),
            Self::ColorFinal => format!("colorfinal"),
        }
    }
    
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mapping {

    // Must be line|scatter|area|bar|text|interval
    pub kind : String,

    pub map : Map,

    // Shared by all mappings
    pub color : Option<String>,

    // Shared by line, bar and interval
    pub width : Option<f64>,
    pub spacing : Option<f64>,

    // Shared by interval/bar
    pub vertical : Option<bool>,
    
    // text-specific
    pub font : Option<String>,

    // Scatter-specific
    pub radius : Option<f64>,

    // Interval-specific
    pub limits : Option<f64>,
   
    // Bar-specific
    pub center : Option<bool>,
    pub origin : Option<f64>,

    // Surface-specific
    pub color_final : Option<String>,
    pub z_start : Option<f64>,
    pub z_end : Option<f64>

}

fn hex_byte_at(full : &str, pos : Range<usize>) -> bool {
    if let Some(sub) = full.get(pos) {
        u8::from_str_radix(sub, 16).is_ok()
    } else {
        false
    }
}

pub(crate) fn validate_color(s : &str) -> bool {
    let has_rgb = s.starts_with("#") && hex_byte_at(s, 1..3) && hex_byte_at(s, 3..5) && hex_byte_at(s, 5..7);
    match s.len() {
        7 => has_rgb,
        9 => has_rgb && hex_byte_at(s, 7..9),
        _ => false
    }
}

impl Mapping {

    pub fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property::Kind, Property::Map];
        if self.color.is_some() {
            props.push(Property::Color);
        }

        // Shared by line, bar and interval
        if self.width.is_some() {
            props.push(Property::Width);
        }
        
        if self.spacing.is_some() {
            props.push(Property::Spacing);
        }

        if self.vertical.is_some() {
            props.push(Property::Vertical);
        }
        
        if self.font.is_some() {
            props.push(Property::Font);
        }

        if self.radius.is_some() {
            props.push(Property::Radius);
        }

        if self.limits.is_some() {
            props.push(Property::Limits);
        }
       
        if self.center.is_some() {
            props.push(Property::Center);
        }
        if self.origin.is_some() {
            props.push(Property::Origin);
        }

        if self.color_final.is_some() {
            props.push(Property::ColorFinal);
        }

        if self.z_start.is_some() {
            props.push(Property::ZStart);
        }

        if self.z_end.is_some() {
            props.push(Property::ZEnd);
        }

        props
    }
    
    // Returns true when this has at least one of the properties shared by the mappig pair.
    // Returns false otherwise.
    pub fn has_shared_property(&self, a : MappingType, b : MappingType) -> bool {
    
        let mut any_prop = false;
        
        let g2 = [MappingType::Bar, MappingType::Interval];
        if a.like_any(&g2) && b.like_any(&g2) {
            any_prop = self.vertical.is_some()
        }
        
        let g1 = [MappingType::Line, MappingType::Bar, MappingType::Interval];
        if a.like_any(&g1) && b.like_any(&g1) {
            any_prop = any_prop || self.width.is_some() || self.spacing.is_some();
        }
        
        any_prop
    }

    pub fn has_specific_property(&self, kind : MappingType) -> bool {
        match kind {
            MappingType::Bar => {
                self.center.is_some() || self.origin.is_some()
            },
            MappingType::Interval => {
                self.limits.is_some()
            },
            MappingType::Text => {
                self.font.is_some()
            },
            MappingType::Scatter => {
                self.radius.is_some()
            },
            _ => false
        }
    }

    pub fn validate(&self) -> Result<(), MappingError> {
        let ty = MappingType::from_str(&self.kind)
            .ok_or(MappingError::InvalidKind(self.kind.to_string()))?;

        if self.map.x.is_none() {
            Err(MappingError::MissingColumn)?;        
        }

        let nx = self.map.x.as_ref().map(|data| data.len() ).unwrap_or(0);
        if let Some(y) = &self.map.y {
            if y.len() != nx {
                Err(MappingError::DataLength{expected : nx, informed : y.len() , column : "y" })?;
            }
        }
        if let Some(z) = &self.map.z {
            if z.len() != nx {
                Err(MappingError::DataLength{expected : nx, informed : z.len(), column : "z" })?;
            }
        }
        if let Some(t) = &self.map.text {
            if t.len() != nx {
                Err(MappingError::DataLength{expected : nx, informed : t.len(), column : "t" })?;
            }
        }
        
        if let Some(color) = &self.color {
            if !validate_color(&color[..]) {
                Err(MappingError::InvalidColor)?;
            }
        }
        match ty {
            MappingType::Line => {
                
                let empty = Map::empty_for_line();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Line) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Scatter => {
            
                let empty = Map::empty_for_scatter();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Scatter) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Text => {
            
                let empty = Map::empty_for_label();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Text) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Area => {
            
                let empty = Map::empty_for_area();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Area) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Interval => {
            
                let empty = Map::empty_for_interval();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Interval) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Bar => {
            
                let empty = Map::empty_for_bar();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                
                for pr in self.properties() {
                    if pr.absent(&MappingType::Bar) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
            MappingType::Surface => {
                let empty = Map::empty_for_surface();
                if !self.map.like(&empty) {
                    return Err(MappingError::DataMapping { expected : empty.description(), informed : self.map.description() });
                }
                for pr in self.properties() {
                    if pr.absent(&MappingType::Surface) {
                        return Err(MappingError::InvalidProperty(pr.name()));
                    }
                }
            },
        }
        Ok(())
    }

}

/// Represents a mapping definition error propagated to the user.
#[derive(thiserror::Error, Debug)]
pub enum MappingError {

    #[error("Missing first mapping data column (x)")]
    MissingColumn,
    
    #[error("Invalid mapping kind: {0} (expected line, scatter, interval, area, label or bar)")]
    InvalidKind(String),

    #[error("Data length mismatch (expected {expected}, but informed {informed} for {column})")]
    DataLength { column : &'static str, expected : usize, informed : usize},
    
    #[error("Invalid data mapping (required {expected} but informed {informed})")]
    DataMapping { expected : String, informed : String },

    #[error("Invalid mapping property: {0}")]
    InvalidProperty(String),

    #[error("Invalid RGB/RGBA color")]
    InvalidColor
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

impl Plot {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PlotBuilder {
        PlotBuilder(Self::default())
    }
    
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if let Some(d) = &self.design {
            d.validate()?;
        }
        if let Some(l) = &self.layout {
            l.validate()?;
        }
        self.x.validate()?;
        self.y.validate()?;
        for m in &self.mappings {
            m.validate()?;
        }
        Ok(())
    }

}

impl fmt::Display for Plot {

    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap() )
    }

}

pub struct PlotBuilder(Plot);

impl PlotBuilder {

    pub fn build(self) -> Plot {
        self.0
    }

    /// Appends a single mapping to this vector.
    pub fn mapping<M>(mut self, mapping : M) -> Self
    where
        M : Into<Mapping>
    {
        self.0.mappings.push(mapping.into());
        self
    }

    /// Appends several mappings to this vector (all of the same time).
    /// Can be called multiple times with different mapping types.
    pub fn mappings<I, M>(mut self, mappings : I) -> Self
    where
        I : IntoIterator<Item=M>,
        M : Into<Mapping>
    {
        for m in mappings {
            self.0.mappings.push(m.into());
        }
        self
    }

    pub fn x(mut self, scale : Scale) -> Self {
        self.0.x = scale;
        self
    }

    pub fn y(mut self, scale : Scale) -> Self {
        self.0.y = scale;
        self
    }

    pub fn design(mut self, design : Design) -> Self {
        self.0.design = Some(design);
        self
    }

    pub fn layout(mut self, layout : Layout) -> Self {
        self.0.layout = Some(layout);
        self
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {

    pub plots : Vec<Plot>,

    pub design : Option<Design>,

    pub layout : Option<Layout>

}

impl Default for Panel {

    fn default() -> Self {
        Panel {
            plots : vec![Plot::default()],
            design : Some(Design::default()),
            layout : Some(Layout::default())
        }
    }
    
}

impl Panel {

    pub fn default_dark() -> Self {
        let mut panel = Self::default();
        panel.design.as_mut().unwrap().bgcolor = format!("#1e1e1eff");
        panel.design.as_mut().unwrap().fgcolor = format!("#454545ff");
        panel
    }

}

impl Panel {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PanelBuilder {
        PanelBuilder(Panel::default())
    }
    
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        self.design.as_ref().map(|d| d.validate() ).unwrap_or(Ok(()))?;
        self.layout.as_ref().map(|l| l.validate() ).unwrap_or(Ok(()))?;
        for pl in &self.plots {
            pl.validate()?;
        }
        Ok(())
    }
    

}

pub struct PanelBuilder(Panel);

impl PanelBuilder {

    pub fn build(self) -> Panel {
        self.0
    }

    pub fn plots<const U : usize>(mut self, plots : [Plot; U]) -> Self {
        self.0.plots = Vec::from(plots);
        self
    }

    pub fn design(mut self, design : Design) -> Self {
        self.0.design = Some(design);
        self
    }

    pub fn layout(mut self, layout : Layout) -> Self {
        self.0.layout = Some(layout);
        self
    }

}

impl fmt::Display for Panel {

    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap() )
    }

}


