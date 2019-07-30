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
    pub width: f64,
    pub weight: f64,

    pub pts: Arc<Vec<InterpPt>>,
}

impl Data for AppState {
    fn same(&self, other: &AppState) -> bool {
        self.width.same(&other.width)
    }
}

#[derive(Clone)]
pub struct InterpPt {
    samples: Vec<InterpSample>,
}

#[derive(Clone)]
pub struct InterpSample {
    pub pt: Point,
    pub width: f64,
    pub weight: f64,
}

// All this should be produced by a derive macro.
pub mod lenses {
    // Discussion: if the inner type were listed first, then
    // the capitalization wouldn't have to be twizzled.
    pub mod calc_state {
        use super::super::AppState;
        use druid::Lens;
        pub struct Width;
        pub struct Weight;

        impl Lens<AppState, f64> for Width {
            fn get<'a>(&self, data: &'a AppState) -> &'a f64 {
                &data.width
            }

            fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut AppState, f: F) -> V {
                f(&mut data.width)
            }
        }

        impl Lens<AppState, f64> for Weight {
            fn get<'a>(&self, data: &'a AppState) -> &'a f64 {
                &data.weight
            }

            fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut AppState, f: F) -> V {
                f(&mut data.weight)
            }
        }
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
        if len == 1 {
            // TODO: I think rbf-interp should handle this case, but since
            // it tries to invert a non-invertible matrix, we work around
            // it here.
            return self.samples[0].pt;
        }
        if len == 2 {
            // TODO: RBF-interp should deal with this too.
            let dot = width * (self.samples[1].width - self.samples[0].width)
                + weight * (self.samples[1].weight - self.samples[0].weight);
            let scale = (self.samples[1].width - self.samples[0].width).powi(2) +
                (self.samples[1].weight - self.samples[0].weight).powi(2);
            let t = dot / scale;
            return self.samples[0].pt.lerp(self.samples[1].pt, t);
        }
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
