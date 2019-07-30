use std::ops::Deref;
use std::sync::Arc;

use druid::kurbo::{Circle, Size};
use druid::piet::{Color, FillRule, RenderContext};
use druid::{
    Action, BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};

use crate::AppState;

#[derive(Default)]
pub struct InterpPane {
    drag_ix: Option<usize>,
}

impl Widget<AppState> for InterpPane {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        data: &AppState,
        _env: &Env,
    ) {
        for pt in data.pts.deref() {
            let circle = Circle::new(*pt, 5.0);
            let brush = paint_ctx.render_ctx.solid_brush(Color::WHITE);
            paint_ctx.render_ctx.fill(circle, &brush, FillRule::NonZero);
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
    ) -> Option<Action> {
        match event {
            Event::MouseDown(e) => {
                println!("mouse down {:?}!", e);
                let pos = e.pos;
                let mut pts = data.pts.deref().clone();
                for (i, pt) in pts.iter().enumerate() {
                    if pt.distance(pos) < 5.0 {
                        self.drag_ix = Some(i);
                        return None;
                    }
                }
                self.drag_ix = Some(pts.len());
                pts.push(pos);
                data.pts = Arc::new(pts);
                ctx.invalidate();
            }
            Event::MouseUp(_e) => {
                self.drag_ix = None;
            }
            Event::MouseMoved(e) => {
                if let Some(drag_ix) = self.drag_ix {
                    let mut pts = data.pts.deref().clone();
                    pts[drag_ix] = e.pos;
                    data.pts = Arc::new(pts);
                    ctx.invalidate();
                }
            }
            _ => (),
        }
        None
    }

     fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: Option<&AppState>, _data: &AppState, _env: &Env) {}
}
