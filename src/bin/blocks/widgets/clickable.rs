use crate::*;
use crate::widgets::*;
use std::ffi::CString;
use raylib::prelude::*;

pub struct Clickable {
    pub rect: Rectangle,
    pub text: String,
    pub ready: bool,
    pub callback: fn(&Self, WidgetResult) -> WidgetResult,
}

impl Clickable {
    pub fn new(
        rect: Rectangle,
        text: String,
        callback: fn(&Self, WidgetResult) -> WidgetResult) -> Self {
        Clickable {
            rect,
            text,
            ready: false,
            callback,
        }
    }

    #[inline]
    pub fn ready(&mut self) { self.ready = true }

    #[inline]
    pub fn unready(&mut self) { self.ready = false }
}

impl WidgetRender for Clickable {
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
