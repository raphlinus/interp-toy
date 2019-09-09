//! UI for the master in a list view

use druid::BoxedWidget;

use crate::app_state::{Master, Shared};

struct MasterWrapper {
    child: BoxedWidget<(Shared, Master)>,
}
