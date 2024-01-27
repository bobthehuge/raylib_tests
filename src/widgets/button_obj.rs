use crate::*;
use std::ffi::{CString};

pub struct ButtonObj {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: Option<CString>,
    visible: bool,
    moving: bool,
}

impl ButtonObj {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: Option<CString>) 
        -> ButtonObj {
        ButtonObj{
            x: x,
            y: y,
            width: width,
            height: height,
            text: text,
            visible: false,
            moving: false,
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle{
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) -> bool {
        d.gui_button(
            self.rect(),
            self.text.as_deref(),
        )
    }
}

impl WidgetVisibility for ButtonObj {
    #[inline]
    fn is_visible(&self) -> bool {
        self.visible
    }

    #[inline]
    fn show(&mut self) {
        self.visible = true;
    }

    #[inline]
    fn hide(&mut self) {
        self.visible = false;
    }

    #[inline]
    fn set_visibility(&mut self, state: bool) {
        self.visible = state;
    }
}

impl WidgetMobility for ButtonObj {
    #[inline]
    fn is_moving(&self) -> bool {
        self.moving
    }

    #[inline]
    fn set_moving(&mut self, state: bool) {
        self.moving = state;
    }
}
