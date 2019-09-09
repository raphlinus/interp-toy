use std::ops::Deref;
use std::sync::Arc;

use nalgebra::DVector;

use rbf_interp::{Basis, Scatter};

use druid::kurbo::Point;

use druid::Data;

#[derive(Clone, Default)]
pub struct AppState {
    // TODO: increasingly aware that we're hardcoding two parameters,
    // this needs to be a variable number, but we're trying to keep
    // complexity down for now.
    pub shared: Shared,

    pub pts: Arc<Vec<InterpPt>>,

    pub masters: Arc<Vec<Master>>,
}

impl Data for AppState {
    fn same(&self, other: &AppState) -> bool {
        self.shared.same(&other.shared)
            && self.pts.same(&other.pts)
            && self.masters.same(&other.masters)
    }
}

impl Data for Shared {
    fn same(&self, other: &Shared) -> bool {
        self.width.same(&other.width) && self.weight.same(&other.weight)
    }
}

#[derive(Clone)]
pub struct InterpPt {
    samples: Vec<InterpSample>,
}

/// This is data that's made available to individual master entries
#[derive(Clone, Default)]
pub struct Shared {
    pub width: f64,
    pub weight: f64,
}

#[derive(Clone)]
pub struct Master {
    pub weight: f64,
}

#[derive(Clone)]
pub struct InterpSample {
    pub pt: Point,
    pub width: f64,
    pub weight: f64,
}

impl Data for Master {
    fn same(&self, other: &Self) -> bool {
        self.weight.same(&other.weight)
    }
}

// All this should be produced by a derive macro.
pub mod lenses {
    // Discussion: if the inner type were listed first, then
    // the capitalization wouldn't have to be twizzled.
    pub mod calc_state {
        use super::super::{AppState, Master};
        use druid::Lens;
        use std::sync::Arc;
        pub struct Width;
        pub struct Weight;
        pub struct Masters;

        // Note: this lens isn't quite right.
        impl Lens<AppState, f64> for Width {
            fn get<'a>(&self, data: &'a AppState) -> &'a f64 {
                &data.shared.width
            }

            fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut AppState, f: F) -> V {
                f(&mut data.shared.width)
            }
        }

        impl Lens<AppState, f64> for Weight {
            fn get<'a>(&self, data: &'a AppState) -> &'a f64 {
                &data.shared.weight
            }

            fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut AppState, f: F) -> V {
                f(&mut data.shared.weight)
            }
        }

        impl Lens<AppState, Arc<Vec<Master>>> for Masters {
            fn get<'a>(&self, data: &'a AppState) -> &'a Arc<Vec<Master>> {
                &data.masters
            }

            fn with_mut<V, F: FnOnce(&mut Arc<Vec<Master>>) -> V>(
                &self,
                data: &mut AppState,
                f: F,
            ) -> V {
                f(&mut data.masters)
            }
        }
    }
}

impl AppState {
    pub fn add_new_master(&mut self) {
        let mut masters = self.masters.deref().to_owned();
        masters.push(Master {
            weight: self.shared.weight,
        });
        self.masters = masters.into();
        println!("adding new master");
    }
}

impl InterpPt {
    pub fn new(pt: Point, width: f64, weight: f64) -> InterpPt {
        let sample = InterpSample { pt, width, weight };
        InterpPt {
            samples: vec![sample],
        }
    }

    pub fn eval(&self, width: f64, weight: f64) -> Point {
        let len = self.samples.len();
        let mut centers = Vec::with_capacity(len);
        let mut vals = Vec::with_capacity(len);
        for sample in &self.samples {
            centers.push(DVector::from_vec(vec![sample.width, sample.weight]));
            vals.push(DVector::from_vec(vec![sample.pt.x, sample.pt.y]));
        }
        let scatter = Scatter::create(centers, vals, Basis::PolyHarmonic(2), 2);
        let params = DVector::from_vec(vec![width, weight]);
        let interp = scatter.eval(params);
        Point::new(interp[0], interp[1])
    }

    /// Update a point, either by adding a new sample or updating an existing
    /// sample that's "close" to the specific params.
    pub fn update(&mut self, pt: Point, width: f64, weight: f64) {
        // Try to find an existing sample to update.
        for sample in &mut self.samples {
            if (sample.width - width).powi(2) + (sample.weight - weight).powi(2) < 0.001 {
                sample.width = width;
                sample.weight = weight;
                sample.pt = pt;
                return;
            }
        }
        let sample = InterpSample { pt, width, weight };
        self.samples.push(sample);
    }
}
