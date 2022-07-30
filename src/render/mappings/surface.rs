// use libxml::tree::node::Node;
use gdk4::RGBA;
use cairo::{Context, MeshCorner};
use super::super::context_mapper::ContextMapper;
use std::collections::HashMap;
// use super::utils;
use super::super::context_mapper::Coord2D;
use super::*;
use cairo::Mesh;
use std::cmp::*;
use super::super::MappingProperty;
use crate::model::MappingType;

#[derive(Clone, Debug)]
pub struct SurfaceMapping {
    x : Vec<f64>,
    y : Vec<f64>,
    z : Vec<f64>,
    z_lims : (f64, f64),
    color : RGBA,
    color_final : RGBA,
    col_names : [String; 3],
    source : String
}

impl Default for SurfaceMapping {

    fn default() -> Self {
        Self {
            color : RGBA::black(),
            color_final : RGBA::white(),
            x : Vec::new(),
            y : Vec::new(),
            z : Vec::new(),
            z_lims : (0.0, 1.0),
            col_names : [String::new(), String::new(), String::new()],
            source : String::new()
        }
    }

}

/// Count everything clock-wise from bottom-left point of the patch.
#[derive(Debug, Clone, Copy)]
struct CoordPatch {
    c0 : Coord2D,
    c1 : Coord2D,
    c2 : Coord2D,
    c3 : Coord2D,
    colors : [(f64, f64, f64); 4]
}

impl SurfaceMapping {

    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>, z : impl IntoIterator<Item=D>) -> Self
    where
        D : AsRef<f64>
    {
        let mut surface : SurfaceMapping = Default::default();
        let x : Vec<_> = x.into_iter().map(|d| *d.as_ref() ).collect();
        let y : Vec<_> = y.into_iter().map(|d| *d.as_ref() ).collect();
        let z : Vec<_> = z.into_iter().map(|d| *d.as_ref() ).collect();
        surface.update_data(vec![x, y, z]);
        surface
    }

    /*pub fn new(node : &Node) -> Result<Self, String> {
        let color = gdk::RGBA{
            red:0.0,
            green:0.0,
            blue:0.0,
            alpha : 1.0
        };
        let color_final = gdk::RGBA{
            red:1.0,
            green:1.0,
            blue:1.0,
            alpha : 1.0
        };
        let x = Vec::<f64>::new();
        let y = x.clone();
        let z = x.clone();
        let z_lims = (0.0, 1.0);
        let col_names = [
            String::from("None"),
            String::from("None"),
            String::from("None")
        ];
        // let interp_task = None;
        let source = String::new();
        let mut mapping = SurfaceMapping{
            color,
            color_final,
            x,
            y,
            z,
            col_names,
            z_lims,
            // _interp_task : interp_task,
            source
        };
        mapping.update_layout(node)?;
        Ok(mapping)
    }*/

    fn create_uniform_coords(mapper : &ContextMapper, n : usize) -> Vec<CoordPatch> {
        let (x_ext, y_ext) = mapper.coord_extensions();
        let mut corner_rows : Vec<Vec<Coord2D>> = Vec::new();
        let bounds = mapper.coord_bounds();
        let corner_x_space = x_ext / n as f64;
        let corner_y_space = y_ext / n as f64;
        for r in 0..(n+1) {
            let mut row = Vec::new();
            for c in 0..(n+1) {
                row.push(Coord2D{
                    x : bounds.3.x + (c as f64)*corner_x_space,
                    y : bounds.3.y + (r as f64)*corner_y_space
                });
            }
            corner_rows.push(row);
        }
        let mut patches : Vec<CoordPatch> = Vec::new();
        let pair_row_iter = corner_rows.iter()
            .zip(corner_rows.iter().skip(1));
        for (top_row, bottom_row) in pair_row_iter {
            for c in 0..(top_row.len() - 1) {
                patches.push(CoordPatch{
                    c0 : bottom_row[c],
                    c1 : top_row[c],
                    c2 : top_row[c+1],
                    c3 : bottom_row[c+1],
                    colors : [(0.0, 0.0, 0.0); 4]
                })
            }
        }
        patches
    }

    /*fn calc_color_ratios(&self, mut patch : CoordPatch, interp_task : &Interpolation2D) -> CoordPatch {
        // TODO use logarithmic color progression
        let red_distance = (self.color_final.red - self.color.red).abs();
        let green_distance = (self.color_final.green - self.color.green).abs();
        let blue_distance = (self.color_final.blue - self.color.blue).abs();
        let mut z0 = interp_task.interpolate_point(patch.c0.x, patch.c0.y);
        z0 /= (self.z_lims.1 - self.z_lims.0).abs();
        let mut z1 = interp_task.interpolate_point(patch.c1.x, patch.c1.y);
        z1 /= (self.z_lims.1 - self.z_lims.0).abs();
        let mut z2 = interp_task.interpolate_point(patch.c2.x, patch.c2.y);
        z2 /= (self.z_lims.1 - self.z_lims.0).abs();
        let mut z3 = interp_task.interpolate_point(patch.c3.x, patch.c3.y);
        z3 /= (self.z_lims.1 - self.z_lims.0).abs();
        for (c, z) in patch.colors.iter_mut().zip([z0, z1, z2, z3].iter()) {
            c.0 = red_distance * z;
            c.1 = green_distance * z;
            c.2 = blue_distance * z;
        }
        patch
    }*/

}

impl Mapping for SurfaceMapping {

    fn clone_boxed(&self) -> Box<dyn Mapping> {
        Box::new(self.clone())
    }

    fn update(&mut self, prop : MappingProperty) -> bool {
        match prop {
            MappingProperty::Line(line) => {

                true
            },
            _ => false
        }
    }

    fn update_from_json(&mut self, mut rep : crate::model::Mapping) {
        // rep.z : Option<Vec<f64>>,
        // TODO check properties of other mappings are None.
        unimplemented!()
    }

    /// The z column maps a ratio between zmin and zmax
    /// into a color ratio between color and color max
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) -> Result<(), Box<dyn Error>> {

        if self.x.len() == 0 {
            return;
        }

        ctx.save();
        ctx.set_source_rgba(
            self.color.red.into(),
            self.color.green.into(),
            self.color.blue.into(),
            self.color.alpha.into()
        );
        // (1) Create uniform mesh of given resolution
        let density = 5;
        let mut unif_patches = SurfaceMapping::create_uniform_coords(mapper, density);
        let (bl, br, _tr, tl) = mapper.coord_bounds();

        // Save coordinates corresponding to the bottom and left part of the plot area.
        let x_coords : Vec<_>= self.x.iter().map(|x| mapper.map(*x, 0.0).x ).collect();
        let y_coords : Vec<_>= self.y.iter().map(|y| mapper.map(0.0, *y).y ).collect();
        /*let task = Interpolation2D::new(
            (bl.x, br.x),
            (tl.y, bl.y),
            density,
            density,
            &x_coords[..],
            &y_coords[..],
            &self.z[..]
        ).expect("Error creating interpolation task");
        //println!("Domain: {:?}", ((bl.x, br.x),(bl.y, tl.y)));
        unif_patches.iter_mut().for_each(|patch| *patch = self.calc_color_ratios(*patch, &task) );
        let mesh = Mesh::new();
        for (_i, patch) in unif_patches.iter().enumerate() {
            mesh.begin_patch();
            mesh.move_to(patch.c0.x, patch.c0.y);
            mesh.line_to(patch.c1.x, patch.c1.y);
            mesh.line_to(patch.c2.x, patch.c2.y);
            mesh.line_to(patch.c3.x, patch.c3.y);
            mesh.line_to(patch.c0.x, patch.c0.y);
            mesh.set_corner_color_rgba(
                MeshCorner::MeshCorner0,
                patch.colors[0].0,
                patch.colors[0].1,
                patch.colors[0].2,
                self.color.alpha
            );
            mesh.set_corner_color_rgba(
                MeshCorner::MeshCorner1,
                patch.colors[1].0,
                patch.colors[1].1,
                patch.colors[1].2,
                self.color.alpha
            );
            mesh.set_corner_color_rgba(
                MeshCorner::MeshCorner2,
                patch.colors[2].0,
                patch.colors[2].1,
                patch.colors[2].2,
                self.color.alpha
            );
            mesh.set_corner_color_rgba(
                MeshCorner::MeshCorner3,
                patch.colors[3].0,
                patch.colors[3].1,
                patch.colors[3].2,
                self.color.alpha
            );
            mesh.end_patch();
        }*/

        //ctx.set_source(&*mesh);
        ctx.paint();
        ctx.restore();
        Ok(())
    }

    /// GSL requires z to be organized as a row-wise matrix, so we
    /// sort the received data within each row and across all rows
    /// before setting it.
    fn update_data(&mut self, mut values : Vec<Vec<f64>>) {
        /*if x.len() != y.len() || x.len() != z.len() || x.len() < 3 {
            println!("Dimension mismatch when updating surface data");
            return;
        }*/
        self.x = values.remove(0);
        self.y = values.remove(0);
        self.z = values.remove(0);
        // println!("setting data = {:?}", self.z);
        /*let mut triples : Vec<(f64, f64, f64)> =
            x.iter().zip(y.iter()).zip(z.iter())
                .map(|((x, y), z)| (*x, *y, *z) )
                .collect();
        let mut x_split = Vec::new();
        let mut y_split = Vec::new();
        for (i, t) in triples.iter().enumerate() {
            if i == 0 || (i != 0 && t.0 != triples[i - 1].0) {
                x_split.push(t.0);
                y_split.push(t.1);
            }
        }

        // Order x (column) values within each row.
        for row in triples.chunks_mut(x.len()) {
            row.sort_by(|triple_1, triple_2| triple_1.0.partial_cmp(&triple_2.0)
                .expect("Invalid f64 comparison at surface data update")
            );
        }

        // Order y (row) values by first value within each point.
        let mut rows : Vec<_> = triples.chunks_mut(x.len()).collect();
        rows.sort_by(|row_0, row_1| row_0[0].1.partial_cmp(&row_1[0].1)
            .expect("Invalid f64 comparison at surface data update")
        );*/
        // Check if all values at x and y slices are the same.
        /*for r in rows {
            x_sorted.push(r[0].0);
            y_sorted.push(r[0].1);
            for (_, _, z) in r {
                z_sorted.push(*z);
            }
        }
        self.x = x_sorted;
        self.y = y_sorted;
        self.z = z_sorted;*/
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
        self.color_final = props.get("final_color")
            .ok_or(format!("final_color property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse final_color property"))?;
        self.color.alpha = props.get("opacity")
            .ok_or(format!("opacity property not found"))?
            .parse::<f64>()
            .map(|op| op / 100.0)
            .map_err(|_| format!("Unable to parse opacity property"))?;
        self.color_final.alpha = self.color.alpha;
        let z_min = props.get("z_min")
            .ok_or(format!("z_min property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse z_min property"))?;
        let z_max = props.get("z_max")
            .ok_or(format!("z_max property not found"))?
            .parse()
            .map_err(|_| format!("Unable to parse z_max property"))?;
        self.z_lims = (z_min, z_max);
        self.col_names[0] = props.get("x")
            .ok_or(format!("x property not found"))?
            .clone();
        self.col_names[1] = props.get("y")
            .ok_or(format!("y property not found"))?
            .clone();
        self.col_names[2] = props.get("z")
            .ok_or(format!("z property not found"))?
            .clone();
        self.source = props.get("source")
            .ok_or(format!("Source property not found"))?
            .clone();
        Ok(())
    }*/

    fn properties(&self) -> HashMap<String, String> {
        let mut properties = MappingType::Surface.default_hash();
        if let Some(e) = properties.get_mut("color") {
            *e = self.color.to_string();
        }
        if let Some(e) = properties.get_mut("final_color") {
            *e = self.color_final.to_string();
        }
        if let Some(e) = properties.get_mut("z_min") {
            *e = self.z_lims.0.to_string();
        }
        if let Some(e) = properties.get_mut("z_max") {
            *e = self.z_lims.1.to_string();
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
        if let Some(e) = properties.get_mut("z") {
            *e = self.col_names[2].clone();
        }
        if let Some(e) = properties.get_mut("source") {
            *e = self.source.clone();
        }
        properties
    }

    fn mapping_type(&self) -> String {
        "surface".into()
    }

    fn get_col_name(&self, col : &str) -> String {
        match col {
            "x" => self.col_names[0].clone(),
            "y" => self.col_names[1].clone(),
            "z" => self.col_names[2].clone(),
            _ => String::new()
        }
    }

    fn get_ordered_col_names(&self) -> Vec<(String,String)> {
        vec![
            (String::from("x"), self.get_col_name("x")),
            (String::from("y"), self.get_col_name("y")),
            (String::from("z"), self.get_col_name("z"))
        ]
    }

    fn get_hash_col_names(&self) -> HashMap<String, String> {
        let mut cols = HashMap::new();
        cols.insert("x".into(), self.col_names[0].clone());
        cols.insert("y".into(), self.col_names[1].clone());
        cols.insert("z".into(), self.col_names[2].clone());
        cols
    }

    fn set_col_name(&mut self, col : &str, name : &str) {
        match col {
            "x" => { self.col_names[0] = name.into(); },
            "y" => { self.col_names[1] = name.into(); },
            "z" => { self.col_names[2] = name.into(); },
            _ => { }
        }
    }

    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {
        if cols.len() != 3 {
            Err("Wrong number of columns.")
        } else {
            self.set_col_name("x", &cols[0]);
            self.set_col_name("y", &cols[1]);
            self.set_col_name("z", &cols[2]);
            Ok(())
        }
    }

    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {
        let xmin = self.x.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal) )?;
        let xmax = self.x.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymin = self.y.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        let ymax = self.y.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;
        Some(((*xmin, *xmax), (*ymin, *ymax)))
    }

    fn set_source(&mut self, source : String) {
        self.source = source;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}

