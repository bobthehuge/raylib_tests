use crate::*;
use std::collections::HashMap;
use crate::widgets::Widget;

pub struct Genv {
    widgets: HashMap<String, Widget>
}

impl Genv {
    pub fn render(&mut self, handle: &mut RaylibDrawHandle) {
        for widget in self.widgets.iter_mut() {
            match widget {
                Widget::Button(b) => {},
                Widget::Image_button(ib) => {},
                Widget::Window_box(wb) => {},
                Widget::Text(t) => {},
            }
        }
    }
}
