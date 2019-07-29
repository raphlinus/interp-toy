use druid::shell::{runloop, WindowBuilder};
use druid::widget::{ActionWrapper, Button, Column, DynLabel, Padding, Slider};
use druid::{LensWrap, UiMain, UiState};

mod app_state;

use app_state::{AppState, lenses};

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
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
    let root = ActionWrapper::new(col, |data: &mut AppState, _env| data.width += 0.1);
    let app_state = AppState::default();
    let state = UiState::new(root, app_state);
    builder.set_title("Hello example");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
