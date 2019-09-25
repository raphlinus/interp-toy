use std::ops::Deref;
use std::sync::Arc;

use nalgebra::DVector;

use glyphstool::NodeType;
use rbf_interp::{Basis, Scatter};

use druid::kurbo::Point;

use druid::Data;

#[derive(Clone, Default)]
pub struct AppState {
    // TODO: increasingly aware that we're hardcoding two parameters,
    // this needs to be a variable number, but we're trying to keep
    // complexity down for now.
    pub shared: Shared,
    pub sel: Option<usize>,

    pub pts: Arc<Vec<InterpPt>>,

    pub masters: Arc<Vec<Master>>,
    pub interp_type: InterpType,

    pub structure: Arc<Vec<Vec<NodeType>>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InterpType {
    Gaussian,
    ThinPlate,
    Linear,
}

impl Default for InterpType {
    fn default() -> Self {
        InterpType::Gaussian
    }
}

impl Data for AppState {
    fn same(&self, other: &AppState) -> bool {
        self.shared.same(&other.shared)
            && self.sel.same(&other.sel)
            && self.pts.same(&other.pts)
            && self.masters.same(&other.masters)
            && self.interp_type.same(&other.interp_type)
            && self.structure.same(&other.structure)
    }
}

impl Data for InterpType {
    fn same(&self, other: &InterpType) -> bool {
        self == other
    }
}

impl Data for Shared {
    fn same(&self, other: &Shared) -> bool {
        self.width.same(&other.width) && self.weight.same(&other.weight)
    }
}

#[derive(Clone, Default)]
pub struct InterpPt {
    pub samples: Vec<InterpSample>,
}

/// This is data that's made available to individual master entries
#[derive(Clone, Default)]
pub struct Shared {
    pub width: f64,
    pub weight: f64,
}

#[derive(Clone)]
pub struct Master {
    pub width: f64,
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
        self.width.same(&other.width) && self.weight.same(&other.weight)
    }
}

// All this should be produced by a derive macro.
pub mod lenses {
    // Discussion: if the inner type were listed first, then
    // the capitalization wouldn't have to be twizzled.
    pub mod app_state {
        use super::super::{AppState, Master};
        use crate::lens2::Lens2;
        use druid::Lens;
        use std::sync::Arc;
        pub struct Width;
        pub struct Weight;
        pub struct Shared;
        pub struct Masters;
        pub struct InterpType;

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

        impl Lens<AppState, super::super::InterpType> for InterpType {
            fn get<'a>(&self, data: &'a AppState) -> &'a super::super::InterpType {
                &data.interp_type
            }

            fn with_mut<V, F: FnOnce(&mut super::super::InterpType) -> V>(
                &self,
                data: &mut AppState,
                f: F,
            ) -> V {
                f(&mut data.interp_type)
            }
        }

        impl Lens2<AppState, super::super::Shared> for Shared {
            fn get<V, F: FnOnce(&super::super::Shared) -> V>(&self, data: &AppState, f: F) -> V {
                f(&data.shared)
            }

            fn with_mut<V, F: FnOnce(&mut super::super::Shared) -> V>(
                &self,
                data: &mut AppState,
                f: F,
            ) -> V {
                f(&mut data.shared)
            }
        }

        impl Lens2<AppState, Arc<Vec<Master>>> for Masters {
            fn get<V, F: FnOnce(&Arc<Vec<Master>>) -> V>(&self, data: &AppState, f: F) -> V {
                f(&data.masters)
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
        if self.is_at_master() {
            // TODO: provide feedback, or, better yet, set enable state of
            // button (see druid#143).
            println!("master already exists");
            return;
        }
        let mut masters = self.masters.deref().to_owned();
        masters.push(Master {
            width: self.shared.width,
            weight: self.shared.weight,
        });
        self.masters = masters.into();
        println!("adding new master");
    }

    pub fn is_at_master(&self) -> bool {
        self.masters
            .iter()
            .any(|master| self.shared.width == master.width && self.shared.weight == master.weight)
    }

    pub fn add_weight(&mut self, weight: f64) {
        let master = Master { width: 0.0, weight };
        Arc::make_mut(&mut self.masters).push(master);
    }

    pub fn set_pts(&mut self, pts: Vec<InterpPt>) {
        *Arc::make_mut(&mut self.pts) = pts;
    }

    pub fn set_structure(&mut self, structure: Vec<Vec<NodeType>>) {
        *Arc::make_mut(&mut self.structure) = structure;
    }
}

impl InterpPt {
    pub fn new(pt: Point, width: f64, weight: f64) -> InterpPt {
        let sample = InterpSample { pt, width, weight };
        InterpPt {
            samples: vec![sample],
        }
    }

    pub fn eval(&self, width: f64, weight: f64, interp_type: InterpType) -> Point {
        let len = self.samples.len();
        let mut centers = Vec::with_capacity(len);
        let mut vals = Vec::with_capacity(len);
        for sample in &self.samples {
            centers.push(DVector::from_vec(vec![sample.width, sample.weight]));
            vals.push(DVector::from_vec(vec![sample.pt.x, sample.pt.y]));
        }
        let (basis, order) = match interp_type {
            InterpType::ThinPlate => (Basis::PolyHarmonic(2), 2),
            // TODO: control over radius
            InterpType::Gaussian => (Basis::Gaussian(1.0), 2),
            InterpType::Linear => (Basis::PolyHarmonic(1), 1),
        };
        let scatter = Scatter::create(centers, vals, basis, order);
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
