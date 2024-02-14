use crate::RaylibDrawHandle;
use std::ffi::CString;
use raylib::prelude::*;

pub struct Clickable {
    pub rect: Rectangle,
    pub text: String,
    pub callback: fn(&Self, WidgetResult),
}

pub enum WidgetResult {
    Bool(bool),
    None,
}

impl Clickable {
    pub fn new(
        rect: Rectangle,
        text: String,
        callback: fn(&Self, WidgetResult)) -> Self {
        Clickable {
            rect,
            text,
            callback,
        }
    }

    pub fn render(&mut self, handle: &mut RaylibDrawHandle) {
        (self.callback)(
            &self,
            WidgetResult::Bool(handle.gui_button(
                self.rect,
                Some(&CString::new(&*self.text).expect("CString::new failed")),
            ))
        )
    }
}
