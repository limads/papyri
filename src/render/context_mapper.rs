use std::ops::Add;



#[derive(Clone, Copy, Debug)]
pub struct Coord2D {
    pub x : f64,
    pub y : f64
}

impl Coord2D {
    pub fn new(x : f64, y : f64) -> Coord2D {
        Coord2D{x, y}
    }

    pub fn distance(&self, other : Coord2D) -> f64 {
        ((self.x - other.x).powf(2.0) +
            (self.y - other.y).powf(2.0)).sqrt()
    }
}

impl Add for Coord2D {

    type Output=Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContextMapper {
    pub xmin : f64,
    pub xmax : f64,
    pub ymin : f64,
    pub ymax : f64,
    pub xlog : bool,
    pub ylog : bool,
    pub xinv : bool,
    pub yinv : bool,
    pub xext : f64,
    pub yext : f64,
    pub w : i32,
    pub h : i32,
}

impl Default for ContextMapper {

    fn default() -> Self {
        let mut mapper = Self {
            xmin : 0.0,
            xmax : 1.0,
            ymin : 0.0,
            ymax : 1.0,
            xlog : false,
            ylog : false,
            xinv : false,
            yinv : false,
            xext : 0.0,
            yext : 0.0,
            w : 800,
            h : 600
        };
        mapper.update();
        mapper
    }

}

impl ContextMapper {

    //fn new() -> ContextMapper {
    //    ContextMapper{..Default::default()}
    //}

    pub fn new(
        xmin : f64, xmax : f64, ymin : f64, ymax : f64,
        xlog : bool, ylog : bool, xinv : bool, yinv : bool)
        -> ContextMapper {

        let (w, h) = (0, 0);
        let (xext, yext) = ContextMapper::calc_ext(
            xmax, xmin, ymax, ymin, xlog, ylog);
        ContextMapper{ xmin, xmax, ymin, ymax,
        xext, yext, w, h, xlog, ylog, xinv, yinv}
    }

    pub fn update(&mut self) {
        let (xext, yext) = Self::calc_ext(self.xmax, self.xmin, self.ymax, self.ymin, self.xlog, self.ylog);
        self.xext = xext;
        self.yext = yext;
    }

    pub fn update_data_extensions(&mut self, xmin : f64, xmax : f64, ymin : f64, ymax : f64) {
        self.xmin = xmin;
        self.xmax = xmax;
        self.ymin = ymin;
        self.ymax = ymax;
        self.update();
    }

    pub fn update_dimensions(&mut self, w : i32, h : i32) {
        self.w = w;
        self.h = h;
        self.update();
    }

    pub fn calc_ext(xmax : f64, xmin : f64, ymax : f64, ymin : f64,
        xlog : bool, ylog : bool) -> (f64, f64) {
        let xext = match xlog {
            true => (xmax.log10() - xmin.log10()).abs(),
            false => (xmax - xmin).abs()
        };
        let yext = match ylog {
            true => (ymax.log10() - ymin.log10()).abs(),
            false => (ymax - ymin).abs()
        };
        (xext, yext)
    }

    pub fn set_mode(&mut self, xinv : bool, xlog : bool, yinv : bool, ylog : bool) {
        self.xlog = xlog;
        self.xinv = xinv;
        self.ylog = ylog;
        self.yinv = yinv;
        self.update();
    }

    pub fn map(&self, x : f64, y : f64) -> Coord2D {
        // This padding works fine for the single-layout plot,
        // but we sould increase it in either or both the horizontal
        // or vertical dimension if they are shared by more than one plot
        // (to leave enough room for labels under the minimum aspect ratio).
        let padw = 0.1*(self.w as f64);
        let padh = 0.1*(self.h as f64);
        let dataw = (self.w as f64) - 2.0*padw;
        let datah = (self.h as f64) - 2.0*padh;

        let xprop = match (self.xlog, self.xinv) {
            (false, false) => (x - self.xmin) / self.xext,
            (false, true)  => (self.xmax - x) / self.xext,
            (true, false)  => (x.log10() - self.xmin.log10()) / self.xext,
            (true, true)   => (self.xmax.log10() - x.log10()) / self.xext
        };
        let yprop = 1.0 - match (self.ylog, self.yinv) { // Here
            (false, false) => (y - self.ymin) / self.yext,
            (false, true)  => (self.ymax - y) / self.yext,
            (true, false)  => (y.log10() - self.ymin.log10()) / self.yext,
            (true, true)   => (self.ymax.log10() - y.log10()) / self.yext
        };

        //println!("{:?} {:?} {:?} {:?}", self.xlog, self.xinv, self.ylog,self.yinv);

        Coord2D::new(padw + dataw*xprop, padh + datah*yprop)
    }

    pub fn check_bounds(&self, x : f64, y : f64) -> bool {
        let x_ok = x >= self.xmin && x <= self.xmax;
        let y_ok = y >= self.ymin && y <= self.ymax;
        /*match self.xinv {
            false => x >= self.xmin && x <= self.xmax,
            true => x <= self.xmin && x >= self.xmax
        };*/
        /*let y_ok = match self.yinv {
            false => y >= self.ymin && y <= self.ymax,
            true => y <= self.ymin && y >= self.ymax
        };*/
        x_ok && y_ok
    }

    pub fn coord_bounds(&self) -> (Coord2D, Coord2D, Coord2D, Coord2D) {
        (
            self.map(self.xmin, self.ymin),
            self.map(self.xmax, self.ymin),
            self.map(self.xmax, self.ymax),
            self.map(self.xmin, self.ymax)
        )
    }

    pub fn data_extensions(&self) -> (f64, f64, f64, f64) {
        (self.xmin, self.xmax, self.ymin, self.ymax)
    }

    pub fn coord_extensions(&self) -> (f64, f64) {
        let x_ext = self.map(self.xmin, self.ymin)
            .distance(self.map(self.xmax, self.ymin));
        let y_ext = self.map(self.xmin, self.ymin)
            .distance(self.map(self.xmin, self.ymax));
        (x_ext, y_ext)
    }

}

// This gives how many log units (on a scale of 0.0 - 10.0) the quantity is at.
fn log_units(a : f64) -> f64 {
    let a = if a == 0.0 { a + f64::EPSILON } else { a };
	(10.0 as f64 ).powf(a.abs().log10() - a.abs().log10().floor() )
}

// Round value to the upward or downward significant decimal digit.
fn round_to(b : f64, place : f64, up : bool) -> f64 {
	let pb = (b * (10.0 as f64 )).powf(place);
	let r_pb = if up {
		pb.ceil()
	} else {
		pb.floor()
	};
	r_pb / (10.0 as f64).powf(place)
}

fn round_to_closest(val : f64, up : bool) -> f64 {
    let val = if val == 0.0 { val + f64::EPSILON } else { val };
    let closest_log = val.abs().log10().floor();
    let rounded_log = round_to(log_units(val), 1.0, up );
    let abs_v = (10.0 as f64).powf(closest_log) * rounded_log;
    if val < 0.0 {
    	(-1.0 * abs_v).round()
    } else {
    	abs_v.round()
    }
}

pub fn round_to_most_extreme(min : f64, max : f64) -> (f64, f64) {
	if min > 0.0 {
		(round_to_closest(min, false), round_to_closest(max, true))
	} else {
		if max > 0.0 {
			(round_to_closest(min, true), round_to_closest(max, true))
		} else {
			(round_to_closest(min, true), round_to_closest(max, false))
		}
	}
}

#[test]
fn scales() {
    println!("{:?}", round_to_most_extreme(-11101.0, -10201.0));
    println!("{:?}", round_to_most_extreme(-11101.0, 10201.0));
    println!("{:?}", round_to_most_extreme(10201.0, 11101.0));
    println!("{:?}", round_to_most_extreme(612.0, 625.0));
}
