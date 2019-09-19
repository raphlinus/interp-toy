use druid::widget::{Button, Column, DynLabel, Padding, RadioGroup, Row, Scroll, Slider};
use druid::{AppLauncher, LensWrap, LocalizedString, Widget, WindowDesc};

mod app_state;
mod interp_pane;
mod lens2;
mod list;
mod master;

use app_state::{lenses, AppState, InterpPt, InterpType};
use interp_pane::InterpPane;
use lens2::{Lens2Wrap, Pair};
use list::List;
use master::MasterItem;

fn build_ui() -> impl Widget<AppState> {
    let pane = InterpPane::default();
    let mut col = Column::new();
    col.add_child(
        Padding::uniform(5.0, LensWrap::new(Slider::new(), lenses::app_state::Weight)),
        0.0,
    );
    let label = DynLabel::new(|data: &AppState, _env| format!("weight: {:.2}", data.shared.weight));
    col.add_child(Padding::uniform(5.0, label), 0.0);
    col.add_child(
        Padding::uniform(5.0, LensWrap::new(Slider::new(), lenses::app_state::Width)),
        0.0,
    );
    let label_wdth =
        DynLabel::new(|data: &AppState, _env| format!("width: {:.2}", data.shared.width));
    col.add_child(Padding::uniform(5.0, label_wdth), 0.0);
    col.add_child(
        LensWrap::new(
            RadioGroup::new(vec![
                (LocalizedString::new("Gaussian"), InterpType::Gaussian),
                (LocalizedString::new("Thin plate"), InterpType::ThinPlate),
                (LocalizedString::new("Linear"), InterpType::Linear),
            ]),
            lenses::app_state::InterpType,
        ),
        0.0,
    );
    let new_master_button = Button::new("New Master", |_ctx, data: &mut AppState, _env| {
        data.add_new_master()
    });
    col.add_child(Padding::uniform(5.0, new_master_button), 0.0);
    col.add_child(
        Scroll::new(Lens2Wrap::new(
            List::new(|| Box::new(MasterItem::new())),
            Pair::new(lenses::app_state::Shared, lenses::app_state::Masters),
        ))
        .vertical(),
        1.0,
    );
    let mut row = Row::new();
    row.add_child(pane, 2.0);
    row.add_child(col, 1.0);
    row
}

fn main() {
    let app_state = AppState::default();
    let title = LocalizedString::new("Interpolation toy");
    let window = WindowDesc::new(build_ui).title(title);
    AppLauncher::with_window(window)
        .launch(app_state)
        .expect("launch failed");
}
