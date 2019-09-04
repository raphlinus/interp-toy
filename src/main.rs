use druid::shell::{runloop, WindowBuilder};
use druid::widget::{ActionWrapper, Button, Column, DynLabel, Padding, Row, Slider};
use druid::{LensWrap, UiMain, UiState};

mod app_state;
mod interp_pane;
mod list;

use app_state::{lenses, AppState, InterpPt};
use interp_pane::InterpPane;

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let pane = InterpPane::default();
    let mut col = Column::new();
    let label = DynLabel::new(|data: &AppState, _env| format!("value: {}", data.width));
    let button = Button::new("increment");
    col.add_child(Padding::uniform(5.0, label), 1.0);
    col.add_child(Padding::uniform(5.0, button), 1.0);
    col.add_child(
        Padding::uniform(
            5.0,
            LensWrap::new(Slider::default(), lenses::calc_state::Width),
        ),
        1.0,
    );
    col.add_child(
        Padding::uniform(
            5.0,
            LensWrap::new(Slider::default(), lenses::calc_state::Weight),
        ),
        1.0,
    );
    let col = ActionWrapper::new(col, |data: &mut AppState, _env| data.width += 0.1);
    let mut row = Row::new();
    row.add_child(pane, 2.0);
    row.add_child(col, 1.0);
    let root = row;

    let app_state = AppState::default();
    let state = UiState::new(root, app_state);
    builder.set_title("Interpolation toy");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
