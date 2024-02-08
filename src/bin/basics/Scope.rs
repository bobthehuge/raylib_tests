use crate::*;
use crate::widgets::{Widget, WidgetRender};
use std::collections::HashMap;

pub enum ScopeType {
    Vector(Vec<Widget>),
    Map(HashMap<(String, i32), Widget>),
}

impl ScopeType {
    pub fn render_vec(&mut self, &mut handle: RaylibDrawHandle) {
        match self {
            ScopeType::Vector(v) => {}
            _ => unreachable!(),
        }
    }
}
