use std::collections::HashMap;
use crate::*;

pub enum RenderResult {
    Bool(bool),
    None(),
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
    ImageButton(ImageButtonObj),
    WindowBox(WindowBoxObj),
    Text(TextObj),
    Vec(Vec<Widget>),
    // HashMap<Label, (Rendering Order, Widget)>
    Map(HashMap<String, (i32, Widget)>),
}

impl<T: WidgetRender> WidgetRender for Vec<T> {
    #[inline]
    fn render(&self, handle: &mut RaylibDrawHandle) -> RenderResult {
        self.iter().for_each(|x| _ = x.render(handle));
        RenderResult::None()
    }
}

impl<T: WidgetVisibility> WidgetVisibility for Vec<T> {
    #[inline]
    fn is_visible(&self) -> bool {
        self.iter().any(|x| x.is_visible())
    }
    #[inline]
    fn show(&mut self) {
        self.iter_mut().for_each(|x| x.show())
    }
    #[inline]
    fn hide(&mut self) {
        self.iter_mut().for_each(|x| x.hide())
    }
    #[inline]
    fn set_visibility(&mut self, state: bool) {
        self.iter_mut().for_each(|x| x.set_visibility(state))
    }
}
