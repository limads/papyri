use serde::{Serialize, Deserialize};
use std::default::Default;
use std::fmt;

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

impl Design {

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mapping {

    // Must be line|scatter|area|bar|surface|text|interval
    pub kind : String,

    // Shared by all mappings
    pub color : Option<String>,
    pub map : Option<Map>,

    // area-specific
    // pub ymin : Option<f64>,
    // pub ymax : Option<f64>,

    // text-specific
    pub font : Option<String>,

    // Scatter-specific
    pub radius : Option<f64>,

    // Line and interval-specific
    pub width : Option<f64>,
    pub spacing : Option<i32>,

    // Interval-specific
    pub limits : Option<f64>,
    pub vertical : Option<bool>,

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


