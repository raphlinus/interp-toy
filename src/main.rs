use std::collections::HashMap;
use std::path::PathBuf;

use kurbo::Affine;

use druid::widget::{Button, Column, DynLabel, Padding, RadioGroup, Row, Scroll, Slider};
use druid::{AppLauncher, LensWrap, LocalizedString, Widget, WindowDesc};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cmd {
    /// Show a blank window, add points manually.
    Blank,
    /// Load a glyph from a font.
    Glyph(GlyphCmd),
}

#[derive(Debug, StructOpt)]
struct GlyphCmd {
    /// A font file in Glyphs format.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The name of the glyph to read.
    glyph: String,
}

mod app_state;
mod interp_pane;
mod lens2;
mod list;
mod master;

use glyphstool::Font;

use app_state::{lenses, AppState, InterpPt, InterpSample, InterpType};
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

fn set_app_state_for_glyph(app_state: &mut AppState, font: &Font, glyphname: &str) {
    let a = Affine::new([0.5, 0.0, 0.0, -0.5, 0.0, 500.0]);
    let mut weight_map = HashMap::new();
    for master in &font.font_master {
        let weight = (master.weight_value - 200) as f64 / 700.0;
        weight_map.insert(master.id.clone(), weight);
        app_state.add_weight(weight);
    }
    if let Some(glyph) = font.get_glyph(glyphname) {
        let mut pts = Vec::new();
        for layer in &glyph.layers {
            if let Some(&weight) = weight_map.get(&layer.layer_id) {
                let mut i = 0;
                for p in layer.paths.as_ref().unwrap() {
                    for n in &p.nodes {
                        if i == pts.len() {
                            pts.push(InterpPt::default());
                        }
                        let sample = InterpSample {
                            pt: a * n.pt,
                            weight,
                            width: 0.0,
                        };
                        pts[i].samples.push(sample);
                        i += 1;
                    }
                }
            }
        }
        app_state.set_pts(pts);
    }
}

fn main() {
    let cmd = Cmd::from_args();

    let mut app_state = AppState::default();
    match cmd {
        Cmd::Glyph(g) => {
            println!("loading glyph {} from file {:?}", g.glyph, g.input);
            let font = Font::load(&g.input).unwrap();
            set_app_state_for_glyph(&mut app_state, &font, &g.glyph);
            println!("masters: {:?}", font.font_master);
        }
        Cmd::Blank => {
            println!("blank app");
        }
    }
    let title = LocalizedString::new("Interpolation toy");
    let window = WindowDesc::new(build_ui).title(title);
    AppLauncher::with_window(window)
        .launch(app_state)
        .expect("launch failed");
}
