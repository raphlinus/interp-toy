use std::sync::Arc;

use druid::kurbo::Point;

use druid::Data;

#[derive(Clone, Default)]
pub struct AppState {
    pub width: f64,
    pub weight: f64,

    pub pts: Arc<Vec<Point>>,
}

impl Data for AppState {
    fn same(&self, other: &AppState) -> bool {
        self.width.same(&other.width)
    }
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
