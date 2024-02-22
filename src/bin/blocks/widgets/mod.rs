pub mod clickable;

use std::ffi::CString;
use std::collections::HashMap;
use raylib::prelude::*;

use crate::Vec2ieeF64;
use crate::clickable::*;

pub trait WidgetRender {
    fn render(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult;
}

pub trait WidgetReady {
    fn ready(&mut self);
}

pub enum WidgetResult {
    Bool(bool),
    None,
}

pub struct Widget {
    pub comps: HashMap<Vec2ieeF64, &dyn WidgetRender>,
    pub ready: bool,
    pub callback: fn(&Self, Vec<WidgetResult>) -> WidgetResult,
}

impl Widget {
    pub fn new(
        callback: fn(&Self, WidgetResult) -> WidgetResult ) -> Self {
        Widget {
            comps: HashMap::new(),
            ready: false,
            callback,
        }
    }
}

impl WidgetRender for Widget {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        let res = self.comps
            .iter_mut()
            .map(|(_,x)| x.render(handle))
            .collect::<Vec<WidgetResult>>();

        if self.ready { (self.callback)(&self, res) }
        else { WidgetResult::Bool(false) }
    }
}
