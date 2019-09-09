use druid::shell::{runloop, WindowBuilder};
use druid::widget::{ActionWrapper, Button, Column, DynLabel, Padding, Row, Scroll, Slider};
use druid::{LensWrap, UiMain, UiState, Widget};

mod app_state;
mod interp_pane;
mod lens2;
mod list;

use app_state::{lenses, AppState, InterpPt, Master};
use interp_pane::InterpPane;
use list::List;

fn build_master_ui() -> impl Widget<Master> {
    // Discussion: we might want spacing as a separate param for the list widget.
    Padding::uniform(
        3.0,
        DynLabel::new(|data: &Master, _env| format!("weight {:.2}", data.weight)),
    )
}

fn build_ui() -> impl Widget<AppState> {
    let pane = InterpPane::default();
    let mut col = Column::new();
    col.add_child(
        Padding::uniform(
            5.0,
            LensWrap::new(Slider::new(), lenses::calc_state::Weight),
        ),
        0.0,
    );
    let label = DynLabel::new(|data: &AppState, _env| format!("weight: {:.2}", data.weight));
    col.add_child(Padding::uniform(5.0, label), 0.0);
    col.add_child(
        Padding::uniform(5.0, LensWrap::new(Slider::new(), lenses::calc_state::Width)),
        0.0,
    );
    let label_wdth = DynLabel::new(|data: &AppState, _env| format!("width: {:.2}", data.width));
    col.add_child(Padding::uniform(5.0, label_wdth), 0.0);
    let new_master_button = Button::new("New Master");
    let new_master_button = ActionWrapper::new(new_master_button, |data: &mut AppState, _env| {
        data.add_new_master()
    });
    col.add_child(Padding::uniform(5.0, new_master_button), 0.0);
    col.add_child(
        Scroll::new(LensWrap::new(
            List::new(|| Box::new(build_master_ui())),
            lenses::calc_state::Masters,
        ))
        .vertical(),
        1.0,
    );
    let col = ActionWrapper::new(col, |data: &mut AppState, _env| data.width += 0.1);
    let mut row = Row::new();
    row.add_child(pane, 2.0);
    row.add_child(col, 1.0);
    row
}

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let root = build_ui();

    let app_state = AppState::default();
    let state = UiState::new(root, app_state);
    builder.set_title("Interpolation toy");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
