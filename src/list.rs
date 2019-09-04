//! Simple list view widget.

use std::sync::Arc;

use druid::kurbo::{Point, Rect, Size};

use druid::{
    Action, BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx,
    Widget, WidgetPod,
};

pub struct List<T: Data, F: FnMut() -> Box<dyn Widget<T>>> {
    closure: F,
    children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data, F: FnMut() -> Box<dyn Widget<T>>> List<T, F> {
    pub fn new(closure: F) -> Self {
        List {
            closure,
            children: Vec::new(),
        }
    }
}

impl<T: Data, F: FnMut() -> Box<dyn Widget<T>>> Widget<Arc<Vec<T>>> for List<T, F> {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        data: &Arc<Vec<T>>,
        env: &Env,
    ) {
        for (child, child_data) in self.children.iter_mut().zip(data.iter()) {
            child.paint_with_offset(paint_ctx, child_data, env);
        }
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Arc<Vec<T>>,
        env: &Env,
    ) -> Size {
        let mut width = bc.min().width;
        let mut y = 0.0;
        for (child, child_data) in self.children.iter_mut().zip(data.iter()) {
            let child_bc = BoxConstraints::new(
                Size::new(bc.min().width, 0.0),
                Size::new(bc.max().width, std::f64::INFINITY),
            );
            let child_size = child.layout(layout_ctx, &child_bc, child_data, env);
            let rect = Rect::from_origin_size(Point::new(0.0, y), child_size);
            child.set_layout_rect(rect);
            width = width.max(child_size.width);
            y += child_size.height;
        }
        bc.constrain(Size::new(width, y))
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut Arc<Vec<T>>,
        env: &Env,
    ) -> Option<Action> {
        let mut action = None;
        let mut new_data = Vec::with_capacity(data.len());
        let mut any_changed = false;
        for (child, child_data) in self.children.iter_mut().zip(data.iter()) {
            let mut d = child_data.to_owned();
            action = Action::merge(action, child.event(event, ctx, &mut d, env));
            if !any_changed && !child_data.same(&d) {
                any_changed = true;
            }
            new_data.push(d);
        }
        if any_changed {
            *data = Arc::new(new_data);
        }
        action
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: Option<&Arc<Vec<T>>>,
        data: &Arc<Vec<T>>,
        env: &Env,
    ) {
        for (child, child_data) in self.children.iter_mut().zip(data.iter()) {
            child.update(ctx, child_data, env);
        }
        let len = self.children.len();
        if len > data.len() {
            self.children.truncate(data.len())
        } else if len < data.len() {
            for child_data in &data[len..] {
                let mut child = WidgetPod::new((self.closure)());
                child.update(ctx, child_data, env);
                self.children.push(child);
            }
        }
    }
}
