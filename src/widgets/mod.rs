use crate::*;

pub enum RenderResult {
    BOOL(bool),
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

pub trait WidgetGeometry {
    fn rect(&self) -> Rectangle;
    fn set_rect(&self, rect: Rectangle);
}

pub mod window_box_obj;
pub mod button_obj;
pub mod image_button_obj;
pub mod text_obj;

pub enum Widget {
    Button(ButtonObj),
    Image_button(ImageButtonObj),
    Window_box(WindowBoxObj),
    Text(TextObj),
}
