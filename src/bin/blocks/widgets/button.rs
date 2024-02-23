use crate::*;
use crate::widgets::*;
use raylib::prelude::*;
use std::ffi::CString;

pub struct Button {
    pub rect: Rectangle,
    pub text: String,
    pub ready: bool,
    pub callback: fn(&Self, WidgetResult) -> WidgetResult,
}

impl Button {
    pub fn new(
        rect: Rectangle,
        text: String,
        callback: fn(&Self, WidgetResult) -> WidgetResult) -> Self {
        Button {
            rect,
            text,
            ready: false,
            callback,
        }
    }

    pub fn ready(&mut self) { self.ready = true }
    pub fn unready(&mut self) { self.ready = false }
}

impl WidgetRender for Button {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        let res = WidgetResult::Bool(handle.gui_button(
            self.rect,
            None,
        ));

        if self.ready { (self.callback)(&self, res) }
        else { WidgetResult::Bool(false) }
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

impl WidgetId for Button {
    fn get_id(&self) -> &String {
        &self.text
    }

    fn set_id(&mut self, text: String) {
        self.text = text
    }
}

impl Widget for Button {
    fn as_widget_render(&self) ->
        Option<&dyn WidgetRender> {
            Some(self as _)
        }

    fn as_widget_render_mut(&mut self) ->
        Option<&mut dyn WidgetRender> {
            Some(self as _)
        }

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

    fn as_widget_id(&self) ->
        Option<&dyn WidgetId> {
            Some(self as _)
        }

    fn as_widget_id_mut(&mut self) ->
        Option<&mut dyn WidgetId> {
            Some(self as _)
        }
}
