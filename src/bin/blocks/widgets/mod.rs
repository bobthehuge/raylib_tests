pub mod block;
pub mod button;
pub mod map;

use raylib::prelude::*;

pub trait WidgetRectangle: Widget {
    fn get_rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);
}

pub trait WidgetCollidable: WidgetRectangle {
    fn check_rect_collision(&self, rect: Rectangle) -> bool;
    fn check_circ_collision(&self, center: Vector2, radius: f32) -> bool;
}

pub trait WidgetState: Widget {
    fn ready(&mut self);
    fn unready(&mut self);
}

pub enum WidgetResult {
    None,
    Bool(bool),
    Vec(Vec<WidgetResult>),
}

#[derive(PartialEq)]
pub enum WidgetType {
    None,
    Block,
    Button,
}

pub trait Widget {
    fn render(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult;
    fn call(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult;

    fn ready(&mut self);
    fn unready(&mut self);
    fn is_ready(&self) -> bool;

    fn get_id(&self) -> &String;
    fn set_id(&mut self, id: String);

    fn as_widget_rectangle(&self)
        -> Option<&dyn WidgetRectangle> { None }

    fn as_widget_rectangle_mut(&mut self)
        -> Option<&mut dyn WidgetRectangle> { None }

    fn as_widget_collidable(&self)
        -> Option<&dyn WidgetCollidable> { None }

    fn as_widget_collidable_mut(&mut self)
        -> Option<&mut dyn WidgetCollidable> { None }
}
