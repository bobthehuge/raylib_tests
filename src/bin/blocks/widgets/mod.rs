pub mod block;
pub mod button;

use raylib::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;

use crate::Vec2ieeF64;
use crate::widgets::button::*;
use crate::widgets::block::*;

pub trait WidgetRender {
    fn render(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult;
}

pub trait WidgetRectangle: WidgetRender {
    fn get_rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);
}

pub trait WidgetCollidable: WidgetRectangle {
    fn check_rect_collision(&self, rect: Rectangle) -> bool;
    fn check_circ_collision(&self, center: Vector2, radius: f32) -> bool;
}

pub trait WidgetId: WidgetRender {
    fn get_id(&self) -> &String;
    fn set_id(&mut self, text: String);
}

pub enum WidgetResult {
    Bool(bool),
    None,
}

// pub enum Widget {
//     Block(Block),
//     Button(Button),
//     Text(Text),
// }

pub trait Widget {
    fn as_widget_render(&self) -> Option<&dyn WidgetRender> { None }
    fn as_widget_render_mut(&mut self) -> Option<&mut dyn WidgetRender> { None }
    fn as_widget_rectangle(&self) -> Option<&dyn WidgetRectangle> { None }
    fn as_widget_rectangle_mut(&mut self) -> Option<&mut dyn WidgetRectangle> { None }
    fn as_widget_collidable(&self) -> Option<&dyn WidgetCollidable> { None }
    fn as_widget_collidable_mut(&mut self) -> Option<&mut dyn WidgetCollidable> { None }
    fn as_widget_id(&self) -> Option<&dyn WidgetId> { None }
    fn as_widget_id_mut(&mut self) -> Option<&mut dyn WidgetId> { None }
}
