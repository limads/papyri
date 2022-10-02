use serde::{Serialize, Deserialize};
use std::default::Default;
use std::fmt;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

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

pub enum MappingType {
    Line,
    Scatter,
    Bar,
    Area,
    // Surface,
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
            // "surface" => Some(MappingType::Surface),
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
    }
}

/*
The design and layout are always applied to the panel. When single plots
carry a layout and design, they refer to the design and layout of the implicit
1-element panel
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Design {
    pub bg_color : String,
    pub grid_color : String,
    pub grid_width : i32,
    pub font : String
}

#[derive(Debug, Default)]
pub struct DesignBuilder(Design);

impl DesignBuilder {

    pub fn build(self) -> Design {
        self.0
    }

    pub fn bg_color(mut self, s : &str) -> Self {
        self.0.bg_color = s.to_string();
        self
    }

    pub fn grid_color(mut self, s : &str) -> Self {
        self.0.grid_color = s.to_string();
        self
    }

    pub fn font(mut self, s : &str) -> Self {
        self.0.font = s.to_string();
        self
    }

    pub fn grid_width(mut self, width : i32) -> Self {
        self.0.grid_width = width;
        self
    }

}

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
        if self.grid_width < 0 || self.grid_width > 50 {
            Err(DesignError::InvalidGridWidth)?;
        }
        if !crate::model::validate_color(&self.grid_color) {
            Err(DesignError::InvalidGridColor)?;
        }
        if !crate::model::validate_color(&self.bg_color) {
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
            bg_color : String::from("#ffffff"),
            grid_color : String::from("#d3d7cf"),
            grid_width : 1,
            font : String::from("Monospace Regular 12")
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Layout { ratio : f64, stacked : bool }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
    pub width : i32,
    pub height : i32,
    pub horizontal_ratio : f64,
    pub vertical_ratio : f64,
    pub split : Option<String>
}

impl Layout {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> LayoutBuilder {
        LayoutBuilder(Layout::default())
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

    pub fn horizontal_ratio(mut self, ratio : f64) -> Self {
        self.0.horizontal_ratio = ratio;
        self
    }

    pub fn vertical_ratio(mut self, ratio : f64) -> Self {
        self.0.vertical_ratio = ratio;
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
            horizontal_ratio : 0.5,
            vertical_ratio : 0.5,
            split : None
        }
    }
}

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
    pub precision : i32,
    pub from : f64,
    pub to : f64,
    pub n_intervals : i32,
    pub log : bool,
    pub invert : bool,
    pub offset : i32,
    pub adjust : Option<String>
}

impl Scale {

    pub fn validate(&self) -> Result<(), ScaleError> {
        if self.from > self.to {
            Err(ScaleError::InvertedRange)?;
        }
        if self.offset < 0 || self.offset > 100 {
            Err(ScaleError::InvalidOffset)?;
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
            adjust : Some(String::from("tight"))
        }
    }
}

pub struct ScaleBuilder(Scale);

impl ScaleBuilder {

    pub fn build(self) -> Scale {
        self.0
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
        self.0.precision = precision;
        self
    }

    pub fn n_intervals(mut self, n_intervals : i32) -> Self {
        self.0.n_intervals = n_intervals;
        self
    }

    pub fn offset(mut self, offset : i32) -> Self {
        self.0.offset = offset;
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
        self.0.log = log;
        self
    }

    pub fn invert(mut self, invert : bool) -> Self {
        self.0.invert = invert;
        self
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Map {

    pub x : Option<Vec<f64>>,
    pub y : Option<Vec<f64>>,

    //surface and area-specific
    pub z : Option<Vec<f64>>,

    // Text-specific
    pub text : Option<Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub map : Map,
    pub width : f64,
    pub spacing : i32,
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

    pub fn spacing(mut self, spacing : i32) -> Self {
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
            map : Map::default(),
            width : 1.0,
            spacing : 1,
            color : String::from("#000000")
        }
    }

}

impl From<Line> for Mapping {

    fn from(line : Line) -> Self {
        let Line { map, width, spacing, color, .. } = line;
        Mapping { kind : String::from("line"), map : Some(map), width : Some(width), spacing : Some(spacing), color : Some(color), ..Default::default() }
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
            map : Map::default(),
            radius : 1.0,
            color : String::from("#000000")
        }
    }

}

impl From<Scatter> for Mapping {

    fn from(scatter : Scatter) -> Self {
        let Scatter { map, radius, color, .. } = scatter;
        Mapping { kind : String::from("scatter"), map : Some(map), radius : Some(radius), color : Some(color), ..Default::default() }
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
            map : Map::default(),
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
            map : Some(map),
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
            map : Map::default(),
            color : String::from("#000000"),
            font : String::from("Monospace Regular 12")
        }
    }

}

impl From<Label> for Mapping {

    fn from(label : Label) -> Self {
        let Label { map, color, font, .. } = label;
        Mapping { kind : String::from("text"), map : Some(map), color : Some(color), font : Some(font), ..Default::default() }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub map : Map,
    pub color : String,
    pub bar_width : f64,
    pub bar_spacing : f64,
    pub origin : f64,
    pub center_anchor : bool,
    pub horizontal : bool,
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
        self.0.bar_width = width;
        self
    }

    pub fn bar_spacing(mut self, bar_spacing : f64) -> Self {
        self.0.bar_spacing = bar_spacing;
        self
    }

    pub fn origin(mut self, origin : f64) -> Self {
        self.0.origin = origin;
        self
    }

    pub fn center_anchor(mut self, center : bool) -> Self {
        self.0.center_anchor = center;
        self
    }

    pub fn horizontal(mut self, horizontal : bool) -> Self {
        self.0.horizontal = horizontal;
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
            map : Map::default(),
            color : String::from("#000000"),
            bar_width : 100.0,
            bar_spacing : 1.0,
            origin : 0.,
            center_anchor : false,
            horizontal : false,
        }
    }

}

impl From<Bar> for Mapping {

    fn from(bar : Bar) -> Self {
        let Bar { map, color, bar_width, bar_spacing, origin, center_anchor, horizontal, .. } = bar;
        Mapping {
            kind : String::from("bar"),
            map : Some(map),
            color : Some(color),
            origin : Some(origin),
            center_anchor :  Some(center_anchor),
            horizontal : Some(horizontal),
            bar_width : Some(bar_width),
            spacing : Some(bar_spacing as i32),
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mapping {

    // Must be line|scatter|area|bar|text|interval
    pub kind : String,

    // Shared by all mappings
    pub color : Option<String>,
    pub map : Option<Map>,

    // Shared by line, bar and interval
    pub width : Option<f64>,
    pub spacing : Option<i32>,

    // text-specific
    pub font : Option<String>,

    // Scatter-specific
    pub radius : Option<f64>,

    // Interval-specific
    pub limits : Option<f64>,
    pub vertical : Option<bool>,

    // Bar-specific
    pub center_anchor : Option<bool>,
    pub horizontal : Option<bool>,
    pub bar_width : Option<f64>,
    pub bar_spacing : Option<f64>,
    pub origin : Option<f64>,

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

    pub fn has_shared_property(&self) -> bool {
        self.width.is_some() || self.spacing.is_some()
    }

    pub fn has_specific_property(&self, kind : MappingType) -> bool {
        match kind {
            MappingType::Bar => {
                self.center_anchor.is_some() || self.horizontal.is_some() ||
                    self.bar_width.is_some() || self.bar_spacing.is_some() || self.origin.is_some()
            },
            MappingType::Interval => {
                self.limits.is_some() || self.vertical.is_some()
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
        let ty = MappingType::from_str(&self.kind).ok_or(MappingError::InvalidKind)?;
        if let Some(map) = &self.map {
            let n = map.x.as_ref().map(|data| data.len() ).unwrap_or(0);
            if let Some(y) = &map.y {
                if y.len() != n {
                    Err(MappingError::DataLength)?;
                }
            }
            if let Some(z) = &map.z {
                if z.len() != n {
                    Err(MappingError::DataLength)?;
                }
            }
            if let Some(t) = &map.text {
                if t.len() != n {
                    Err(MappingError::DataLength)?;
                }
            }
        }
        if let Some(color) = &self.color {
            if !validate_color(&color[..]) {
                Err(MappingError::InvalidColor)?;
            }
        }
        match ty {
            MappingType::Line => {
                if self.has_specific_property(MappingType::Scatter) || self.has_specific_property(MappingType::Text) ||
                self.has_specific_property(MappingType::Bar) || self.has_specific_property(MappingType::Interval) {
                    Err(MappingError::InvalidProperty)?;
                }
            },
            MappingType::Scatter => {
                if self.has_specific_property(MappingType::Text) || self.has_specific_property(MappingType::Bar) ||
                self.has_specific_property(MappingType::Interval) {
                    Err(MappingError::InvalidProperty)?;
                }
                if self.has_shared_property() {
                    Err(MappingError::InvalidProperty)?;
                }
            },
            MappingType::Text => {
                if self.has_specific_property(MappingType::Scatter) || self.has_specific_property(MappingType::Bar) ||
                self.has_specific_property(MappingType::Interval) {
                    Err(MappingError::InvalidProperty)?;
                }
                if self.has_shared_property() {
                    Err(MappingError::InvalidProperty)?;
                }
            },
            MappingType::Area => {
                if self.has_specific_property(MappingType::Scatter) || self.has_specific_property(MappingType::Text) ||
                self.has_specific_property(MappingType::Bar) || self.has_specific_property(MappingType::Interval) {
                    Err(MappingError::InvalidProperty)?;
                }
                if self.has_shared_property() {
                    Err(MappingError::InvalidProperty)?;
                }
            },
            MappingType::Interval => {
                if self.has_specific_property(MappingType::Scatter) || self.has_specific_property(MappingType::Text) ||
                self.has_specific_property(MappingType::Bar) {
                    Err(MappingError::InvalidProperty)?;
                }
            },
            MappingType::Bar => {
                if self.has_specific_property(MappingType::Scatter) || self.has_specific_property(MappingType::Text) ||
                self.has_specific_property(MappingType::Interval) {
                    Err(MappingError::InvalidProperty)?;
                }
            }
        }
        Ok(())
    }

}

#[derive(thiserror::Error, Debug)]
pub enum MappingError {

    #[error("Invalid kind")]
    InvalidKind,

    #[error("Data length")]
    DataLength,

    #[error("Invalid property")]
    InvalidProperty,

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
    pub fn mappings<M, const U : usize>(mut self, mappings : [M; U]) -> Self
    where
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

    // TODO add this field and derive deserialize here manually.
    // pub elements : Vec<Either<Box<Panel>, Plot>>,
    pub elements : Vec<Plot>,

    pub design : Option<Design>,

    pub layout : Option<Layout>

}

impl Default for Panel {

    fn default() -> Self {
        Panel {
            elements : Vec::new(),
            design : Some(Design::default()),
            layout : Some(Layout::default())
        }
    }
    
}

impl Panel {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PanelBuilder {
        PanelBuilder(Panel::default())
    }

}

pub struct PanelBuilder(Panel);

impl PanelBuilder {

    pub fn build(self) -> Panel {
        self.0
    }

    pub fn plots<const U : usize>(mut self, plots : [Plot; U]) -> Self {
        self.0.elements = Vec::from(plots);
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


