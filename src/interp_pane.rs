use std::ops::Deref;
use std::sync::Arc;

use glyphstool::NodeType;

use druid::kurbo::{BezPath, Circle, Point, Size};
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

fn reconstruct_path(pts: &[Point], structure: &[Vec<NodeType>]) -> BezPath {
    let mut bez_path = BezPath::new();
    let mut j = 0;
    for subpath in structure {
        let mut start_ix = 0;
        while subpath[start_ix] == NodeType::OffCurve {
            start_ix += 1;
        }
        let n = subpath.len();
        bez_path.move_to(pts[j + start_ix]);
        let mut ctrl_pts = Vec::with_capacity(2);
        for i in 0..n {
            let ix = (start_ix + i + 1) % n;
            let node_type = subpath[ix];
            let p = pts[j + ix];
            match node_type {
                NodeType::Line => bez_path.line_to(p),
                NodeType::OffCurve => ctrl_pts.push(p),
                NodeType::Curve | NodeType::CurveSmooth => {
                    bez_path.curve_to(ctrl_pts[0], ctrl_pts[1], p);
                    ctrl_pts.clear();
                }
            }
        }
        bez_path.close_path();
        j += n;
    }
    bez_path
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
        let pts: Vec<_> = data
            .pts
            .iter()
            .map(|pt| pt.eval(width, weight, data.interp_type))
            .collect();
        let fill_color = Color::WHITE;
        let path = reconstruct_path(&pts, &data.structure);
        paint_ctx.fill(path, &fill_color);
        for i in 0..pts.len() {
            let fg_color = match pt_state {
                PtState::CanAddMaster => Color::rgb8(0x80, 0x80, 0xff),
                PtState::Interpolated => Color::rgb8(0x80, 0x80, 0xff).with_alpha(0.8),
                _ => Color::rgb(0xff, 0, 0),
            };
            let is_selected = data.sel == Some(i);
            let interp = pts[i];
            let radius = if is_selected { 3.0 } else { 2.0 };
            let circle = Circle::new(interp, radius);
            paint_ctx.fill(circle, &fg_color);
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

    fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
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
