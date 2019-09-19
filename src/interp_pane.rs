use std::ops::Deref;
use std::sync::Arc;

use druid::kurbo::{Circle, Size};
use druid::piet::{Color, RenderContext};
use druid::{
    BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};

use crate::AppState;

use crate::InterpPt;

#[derive(Default)]
pub struct InterpPane {
    drag_ix: Option<usize>,
}

enum PtState {
    /// Point is interpolated and can't be dragged.
    Interpolated,
    /// Point doesn't have a master at the current params, but params are at a master.
    CanAddMaster,
    /// Point has a master at the current params.
    IsMaster,
}

impl Widget<AppState> for InterpPane {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        data: &AppState,
        _env: &Env,
    ) {
        let width = data.shared.width;
        let weight = data.shared.weight;
        let pt_state = if data.is_at_master() {
            PtState::CanAddMaster
        } else {
            PtState::Interpolated
        };
        for (i, pt) in data.pts.iter().enumerate() {
            let fg_color = match pt_state {
                PtState::CanAddMaster => Color::WHITE,
                PtState::Interpolated => Color::WHITE.with_alpha(0.5),
                _ => Color::rgb(0xff, 0, 0),
            };
            let is_selected = data.sel == Some(i);
            let interp = pt.eval(width, weight, data.interp_type);
            let radius = if is_selected {
                6.0
            } else {
                5.0
            };
            let circle = Circle::new(interp, radius);
            paint_ctx.render_ctx.fill(circle, &fg_color);
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.constrain((100.0, 100.0))
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut AppState,
        _env: &Env,
    ) {
        let width = data.shared.width;
        let weight = data.shared.weight;
        match event {
            Event::MouseDown(e) => {
                println!("mouse down {:?}!", e);
                let pos = e.pos;
                let mut pts = data.pts.deref().clone();
                for (i, pt) in pts.iter().enumerate() {
                    let interp = pt.eval(width, weight, data.interp_type);
                    if interp.distance(pos) < 5.0 {
                        self.drag_ix = Some(i);
                        data.sel = Some(i);
                        return;
                    }
                }
                self.drag_ix = Some(pts.len());
                data.sel = self.drag_ix;
                let pt = InterpPt::new(pos, width, weight);
                pts.push(pt);
                data.pts = Arc::new(pts);
                ctx.invalidate();
            }
            Event::MouseUp(_e) => {
                self.drag_ix = None;
            }
            Event::MouseMoved(e) => {
                if let Some(drag_ix) = self.drag_ix {
                    let mut pts = data.pts.deref().clone();
                    pts[drag_ix].update(e.pos, data.shared.width, data.shared.weight);
                    data.pts = Arc::new(pts);
                    ctx.invalidate();
                }
            }
            _ => (),
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: Option<&AppState>,
        _data: &AppState,
        _env: &Env,
    ) {
    }
}
