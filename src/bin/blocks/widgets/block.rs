use crate::*;
use crate::widgets::*;
use raylib::prelude::*;
use std::ffi::CString;

pub struct Block {
    pub comps: HashMap<Vec2ieeF64, Box<dyn Widget>>,
    pub ready: bool,
    pub callback: fn(&Self, Vec<WidgetResult>) -> WidgetResult,
}

impl Block {
    pub fn new(
        callback: fn(&Self, Vec<WidgetResult>) -> WidgetResult ) -> Self {
        Block {
            comps: HashMap::new(),
            ready: false,
            callback,
        }
    }
}

impl WidgetRender for Block {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        let res = self.comps
            .iter_mut()
            .filter_map(|(_,x)| x.as_widget_render())
            .map(|x| x.render(handle))
            .collect::<Vec<WidgetResult>>();

        if self.ready { (self.callback)(&self, res) }
        else { WidgetResult::Bool(false) }
    }
}
