//! UI for the master in a list view

use druid::kurbo::{Point, Rect, Size};

use druid::{
    BaseState, BoxConstraints, BoxedWidget, Env, Event, EventCtx, LayoutCtx, PaintCtx,
    UpdateCtx, Widget, WidgetPod,
};

use druid::widget::{DynLabel, Padding};

use crate::app_state::{Master, Shared};

pub struct MasterItem {
    child: BoxedWidget<(Shared, Master)>,
}

impl MasterItem {
    pub fn new() -> Self {
        // Discussion: we might want spacing as a separate param for the list widget.
        let child = Padding::uniform(
            3.0,
            DynLabel::new(|data: &(Shared, Master), _env| {
                format!("weight {:.2} width {:.2}", data.1.weight, data.1.width)
            }),
        );
        MasterItem {
            child: WidgetPod::new(child).boxed(),
        }
    }
}

impl Widget<(Shared, Master)> for MasterItem {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        data: &(Shared, Master),
        env: &Env,
    ) {
        self.child.paint_with_offset(paint_ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &(Shared, Master),
        env: &Env,
    ) -> Size {
        let size = self.child.layout(layout_ctx, &bc, data, env);
        self.child
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        size
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut (Shared, Master),
        env: &Env,
    ) {
        match event {
            Event::MouseDown(_) => {
                data.0.weight = data.1.weight;
                data.0.width = data.1.width;
                return;
            }
            _ => (),
        }
        self.child.event(event, ctx, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: Option<&(Shared, Master)>,
        data: &(Shared, Master),
        env: &Env,
    ) {
        self.child.update(ctx, data, env);
    }
}
