use std::f64::consts::PI;
use std::ops::Deref;
use std::sync::Arc;

use druid::kurbo::{Circle, Line, Point, Size, Vec2};
use druid::piet::{Color, FillRule, RenderContext};
use druid::{
    Action, BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};

use crate::AppState;

pub struct InterpPane;

impl Widget<AppState> for InterpPane {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        data: &AppState,
        _env: &Env,
    ) {
        let t = 0.0;
        let center = Point::new(50.0, 50.0);
        let ambit = center + 45.0 * Vec2::from_angle((0.75 + t) * 2.0 * PI);
        let brush = paint_ctx.render_ctx.solid_brush(Color::WHITE);
        paint_ctx
            .render_ctx
            .stroke(Line::new(center, ambit), &brush, 1.0, None);
        for pt in data.pts.deref() {
            let circle = Circle::new(*pt, 5.0);
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
                let mut pts = data.pts.deref().clone();
                pts.push(e.pos);
                data.pts = Arc::new(pts);
                ctx.invalidate();
            }
            _ => (),
        }
        None
    }

     fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: Option<&AppState>, _data: &AppState, _env: &Env) {}
}
