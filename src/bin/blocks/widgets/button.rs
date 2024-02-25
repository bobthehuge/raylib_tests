use crate::widgets::*;
use raylib::prelude::*;

pub struct Button {
    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub callback: fn(&Self, &WidgetResult),
}

impl Button {
    pub fn new(
        rect: Rectangle,
        id: String,
        callback: fn(&Self, &WidgetResult)
    ) -> Self {
        Self {
            rect,
            id,
            ready: false,
            callback,
        }
    }
}

impl WidgetRectangle for Button {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for Button {
    fn check_rect_collision(&self, rect: Rectangle) -> bool {
        self.rect.check_collision_recs(&rect)
    }

    fn check_circ_collision(&self, center: Vector2, radius: f32) -> bool {
        self.rect.check_collision_circle_rec(center, radius)
    }
}

impl Widget for Button {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        WidgetResult::Bool(handle.gui_button(
            self.rect,
            None,
        ))
    }

    fn call(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult{
        let res = self.render(handle);
        if self.is_ready() { (self.callback)(&self, &res); }
        res
    }

    fn ready(&mut self) { self.ready = true }
    fn unready(&mut self) { self.ready = false }
    fn is_ready(&self) -> bool { self.ready }
    fn get_id(&self) -> &String { &self.id }
    fn set_id(&mut self, id: String) { self.id = id }

    fn as_widget_rectangle(&self) ->
        Option<&dyn WidgetRectangle> {
            Some(self as _)
        }

    fn as_widget_rectangle_mut(&mut self) ->
        Option<&mut dyn WidgetRectangle> {
            Some(self as _)
        }

    fn as_widget_collidable(&self) ->
        Option<&dyn WidgetCollidable> {
            Some(self as _)
        }

    fn as_widget_collidable_mut(&mut self) ->
        Option<&mut dyn WidgetCollidable> {
            Some(self as _)
        }
}
