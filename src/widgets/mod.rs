use crate::*;

pub enum RenderResult {
    BOOL(bool),
    I32(i32),
    NONE(),
}

pub trait WidgetRender {
    fn render(&self, handle: &mut RaylibDrawHandle) -> RenderResult;
}

pub trait WidgetVisibility {
    fn is_visible(&self) -> bool;
    fn show(&mut self);
    fn hide(&mut self);
    fn set_visibility(&mut self, state: bool);
}

pub trait WidgetMobility {
    fn is_moving(&self) -> bool;
    fn set_moving(&mut self, state: bool);
}

pub mod window_box_obj;
pub mod button_obj;
pub mod image_button_obj;
pub mod text_obj;